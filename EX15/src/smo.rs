use std::collections::HashMap;

use std::sync::Arc;
use nalgebra::{DMatrix, DVector, Dyn, OVector};
use ndarray::{Array1, Array2};
use plotters::coord::Shift;
use plotters::prelude::*;
use crate::config::{SMO_CONFIG};


pub struct SMO {
    lambda: i32, // Интенсивность потока заявок
    mu: i32,     // Интенсивность обработки одним офицером
    num_channels: i32, // Количество офицеров
    queue_size: i32, // Ограничение на размер очереди
    initial_state: Arc<HashMap<&'static str, i32>>, // Начальное состояние
    time: i32, // Время
    num_iterations: i32, // Количество итерации
    step_size: f64 // Шаг
}

impl SMO {
    pub fn new(lambda: i32,
               mu: i32,
               num_channels: i32,
               queue_size: i32,
               initial_state: Arc<HashMap<&'static str, i32>>,
               time: i32,
               num_iterations: i32,
               step_size: f64

    ) -> SMO {

        SMO {
            lambda,
            mu,
            num_channels,
            queue_size,
            initial_state,
            time,
            num_iterations,
            step_size
        }
    }

    pub fn plot_state_graph(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dimensions = (1024, 768);
        let root_area = BitMapBackend::new("smo_states.png", dimensions).into_drawing_area();
        root_area.fill(&WHITE)?;

        let states = self.initial_state.len() as i32;
        let step_x = dimensions.0 as f32 / states as f32;
        let step_y = dimensions.1 as f32 / 2.0;
        let rect_height = 40.0;
        let rect_width = step_x - 80.0;
        let arrow_height = 10;
        let total_width = (states - 1) as f32 * step_x + rect_width;
        let offset = (dimensions.0 as f32 - total_width) / 2.0;

        // State labels
        for i in 0..states as usize {
            let x = i as f32 * step_x + offset;
            let state_label = format!("S{}", i);

            // Сначала рисуем контур прямоугольника
            root_area.draw(&Rectangle::new(
                [(x as i32, (step_y - rect_height / 2.0) as i32), ((x + rect_width) as i32, (step_y + rect_height / 2.0) as i32)],
                *&BLACK.mix(1.0).stroke_width(2),
            ))?;
            root_area.draw(&Rectangle::new(
                [(x as i32 + 1, (step_y - rect_height / 2.0) as i32 + 1), ((x + rect_width) as i32 - 1, (step_y + rect_height / 2.0) as i32 - 1)],
                *&WHITE.mix(1.0).filled(),
            ))?;


            // Рисуем текст
            let text_style = TextStyle::from(("sans-serif", 20).into_font()).color(&BLACK);
            root_area.draw_text(
                &state_label,
                &text_style,
                (x as i32 + 10, (step_y - 5.0) as i32),
            )?;

            // Рисуем адаптивные стрелки
            if i < states as usize - 1 {
                let next_rect_start_x = (i + 1) as f32 * step_x + offset;
                let dynamic_arrow_length = (next_rect_start_x - (x + rect_width)).max(30.0);
                let arrow_start_x = x as i32 + rect_width as i32;
                let arrow_end_x = arrow_start_x + dynamic_arrow_length as i32 - arrow_height;
                let mid_arrow_x = arrow_start_x + dynamic_arrow_length as i32 / 2;

                // Синяя стрелка
                root_area.draw(&PathElement::new(
                    vec![(arrow_start_x, step_y as i32 - rect_height as i32 / 4), (arrow_end_x, step_y as i32 - rect_height as i32 / 4)],
                    *&BLUE.stroke_width(2),
                ))?;
                root_area.draw(&Polygon::new(
                    vec![(arrow_end_x, step_y as i32 - rect_height as i32 / 4 - arrow_height / 2), (arrow_end_x, step_y as i32 - rect_height as i32 / 4 + arrow_height / 2), (arrow_end_x + arrow_height, step_y as i32 - rect_height as i32 / 4)],
                    *&BLUE.filled(),
                ))?;

                root_area.draw_text(
                    &format!("λ = {}", self.lambda),
                    &text_style.color(&BLUE),
                    (mid_arrow_x - 20, (step_y - rect_height / 2.0 - 40.0) as i32), // Смещение текста на 50 пикселей вверх от середины стрелки
                )?;
            }

            if i > 0 {
                let previous_rect_end_x = (i - 1) as f32 * step_x + rect_width + offset;
                let dynamic_arrow_length = (x - previous_rect_end_x).max(30.0);
                let arrow_start_x = x as i32;
                let arrow_end_x = arrow_start_x - dynamic_arrow_length as i32 + arrow_height;

                let mid_arrow_x = arrow_end_x + dynamic_arrow_length as i32 / 2;
                let mu_value = i * self.mu as usize;

                // Красная стрелка
                root_area.draw(&PathElement::new(
                    vec![(arrow_start_x, step_y as i32 + rect_height as i32 / 4), (arrow_end_x, step_y as i32 + rect_height as i32 / 4)],
                    *&RED.stroke_width(2),
                ))?;

                root_area.draw(&Polygon::new(
                    vec![(arrow_end_x, step_y as i32 + rect_height as i32 / 4 - arrow_height / 2), (arrow_end_x, step_y as i32 + rect_height as i32 / 4 + arrow_height / 2), (arrow_end_x - arrow_height, step_y as i32 + rect_height as i32 / 4)],
                    *&RED.filled(),
                ))?;

                root_area.draw_text(
                    &format!("μ = {:.1}", mu_value),
                    &text_style.color(&RED),
                    (mid_arrow_x - 20, (step_y + rect_height / 2.0 + 20.0) as i32), // Смещение текста на 30 пикселей вниз от середины стрелки
                )?;
            }
        }

        root_area.present()?;

        Ok(())
    }

    pub fn generate_kolmogorov_matrix(&self) -> Vec<Vec<i32>> {
        let lambda = self.lambda;
        let mu = self.mu;
        let num_channels_i32 = self.num_channels;
        let num_channels_usize = self.num_channels as usize;
        let queue_size = self.queue_size as usize;
        let queue_max_index = num_channels_usize + queue_size;
        let number_of_states = queue_max_index + 1;

        let mut matrix = vec![vec![0; number_of_states]; number_of_states];

        // Обработка для каналов
        (0..num_channels_usize).for_each(|i| {
            (0..=i).for_each(|j| {
                matrix[i][j] = match j {
                    j if i > 0 && j == i - 1 => lambda,
                    j if j == i => - (lambda + (j as i32) * mu),
                    j if j == i + 1 => (j as i32) * mu,
                    _ => 0,
                };
            });
        });

        // Обработка для очереди
        (num_channels_usize..=queue_max_index).for_each(|i| {
            (num_channels_usize..=queue_max_index).for_each(|j| {
                matrix[i][j] = match j {
                    j if j == i - 1 => lambda,
                    j if j == i => - (lambda + num_channels_i32 * mu),
                    j if j == i + 1 && i != queue_max_index => num_channels_i32 * mu,
                    _ => 0,
                };
            });
        });

        matrix
    }

    fn initial_state_to_dvector(initial_state: Arc<HashMap<&'static str, i32>>) -> DVector<f64> {
        DVector::from_iterator(
            initial_state.len(),
            initial_state
                .values()
                .map(|&val| val as f64)
        )
    }

    fn initial_state_to_ovector(initial_state: Arc<HashMap<&'static str, i32>>) -> OVector<f64, Dyn> {
        let values: Vec<f64> = initial_state
            .iter()
            .map(|(_key, &value)| value as f64)
            .collect();

        OVector::<f64, Dyn>::from_column_slice(&values)
    }

    // Функция для преобразования Vec<Vec<i32>> в DMatrix<f64>
    fn kolmogorov_matrix_to_dmatrix(matrix: Vec<Vec<i32>>) -> DMatrix<f64> {
        let rows = matrix.len();
        let cols = matrix.first().map_or(0, Vec::len);

        DMatrix::from_iterator(
            rows, cols,
            matrix.into_iter().flatten().map(|val| val as f64)
        )
    }

    pub fn multiply_matrix_vector(&self, kolmogorov_matrix: Vec<Vec<i32>>, initial_state: Arc<HashMap<&'static str, i32>>) -> DVector<f64> {
        let b = Self::kolmogorov_matrix_to_dmatrix(kolmogorov_matrix);
        let x = Self::initial_state_to_dvector(initial_state);

        b * x
    }

    pub fn integrate_system(&self, f_tx: DVector<f64>) -> Result<Vec<(f64, OVector<f64, Dyn>)>, &'static str> {
        let T_0 = 0.0; // Начальное время
        let T_1 = self.time as f64; // Конечное время
        let n = self.num_iterations; // Число шагов
        let step_size = self.step_size; // Шаг

        let initial_state = Self::initial_state_to_ovector(
            Arc::clone(&SMO_CONFIG.initial_state)
        );

        //let mut rk4 = Rk4::new(f_tx, T_0, initial_state, T_1, step_size);

        // Определение системы уравнений
        let system = move |t: f64, y: &OVector<f64, Dyn>, dy: &mut OVector<f64, Dyn>| {
            *dy = &DVector::from_column_slice(&f_tx.iter().map(|&val| val).collect::<Vec<f64>>()) * y;
        };

        // Создание интегратора RK4
        let mut rk4 = Rk4::new(system, T_0, initial_state, T_1, step_size);

        // Интегрирование
        //rk4.integrate()
            //.map_err(|_| "Ошибка при интегрировании")
            //.map(|_| rk4.y_out().iter().map(|y| (y.0, y.1.clone())).collect()) // Возвращаем результаты интегрирования
    }




}










