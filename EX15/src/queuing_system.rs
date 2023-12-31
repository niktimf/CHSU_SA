
use std::sync::Arc;
use nalgebra::{DMatrix, DVector};
use std::cmp::Ordering::{Equal, Greater, Less};

use plotters::prelude::*;
use crate::config::{QUEUING_SYSTEM_CONFIG};


pub struct QueuingSystem {
    pub lambda_rate: i32, // Интенсивность потока заявок
    pub mu_rate: i32,     // Интенсивность обработки одним офицером
    pub num_channels: i32, // Количество офицеров
    pub queue_size: i32, // Ограничение на размер очереди
    pub initial_state: Arc<Vec<(&'static str, i32)>>, // Начальное состояние
    pub time: i32, // Время
    pub num_iterations: i32, // Количество итерации
    pub step_size: f64 // Шаг
}

impl QueuingSystem {
    pub fn new(lambda_rate: i32,
               mu_rate: i32,
               num_channels: i32,
               queue_size: i32,
               initial_state: Arc<Vec<(&'static str, i32)>>,
               time: i32,
               num_iterations: i32,
               step_size: f64

    ) -> QueuingSystem {

        QueuingSystem {
            lambda_rate,
            mu_rate,
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
        let root_area = BitMapBackend::new("queuing_system_states.png", dimensions).into_drawing_area();
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
                    &format!("λ = {}", self.lambda_rate),
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
                let mu_rate_value = i * self.mu_rate as usize;

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
                    &format!("μ = {:.1}", mu_rate_value),
                    &text_style.color(&RED),
                    (mid_arrow_x - 20, (step_y + rect_height / 2.0 + 20.0) as i32), // Смещение текста на 30 пикселей вниз от середины стрелки
                )?;
            }
        }

        root_area.present()?;

        Ok(())
    }

    pub fn generate_kolmogorov_matrix(&self) -> Vec<Vec<i32>> {
        let lambda_rate = self.lambda_rate;
        let mu_rate = self.mu_rate;
        let num_channels_i32 = self.num_channels;
        let num_channels_usize = self.num_channels as usize;
        let queue_size = self.queue_size as usize;
        let queue_max_index = num_channels_usize + queue_size;
        let number_of_states = queue_max_index + 1;

        (0..number_of_states).map(|i| {
            (0..number_of_states).map(|j| {
                match i.cmp(&j) {
                    Equal => match i {
                        0 => - lambda_rate,
                        _ if i < num_channels_usize => - (lambda_rate + i as i32 * mu_rate),
                        _ => - (lambda_rate + num_channels_i32 * mu_rate),
                    },
                    Less => match j {
                        j if j == i + 1 => if i < num_channels_usize { (i as i32 + 1) * mu_rate } else { num_channels_i32 * mu_rate },
                        _ => 0,
                    },
                    Greater => if j == i - 1 { lambda_rate } else { 0 },
                }
            }).collect()
        }).collect()
    }

    fn initial_state_to_dvector(initial_state: Arc<Vec<(&'static str, i32)>>) -> DVector<f64> {
        let values: Vec<f64> = initial_state
            .iter()
            .map(|(_key, value)| *value as f64)
            .collect();

        DVector::from_vec(values)
    }

    // Функция для преобразования Vec<Vec<i32>> в DMatrix<f64>
    fn kolmogorov_matrix_to_dmatrix(matrix: Vec<Vec<i32>>) -> DMatrix<f64> {
        let rows = matrix.len();
        let cols = matrix.first().map_or(0, Vec::len);

        let flat_matrix: Vec<f64> = matrix.into_iter()
            .flatten()
            .map(|val| val as f64)
            .collect();

        DMatrix::from_row_slice(rows, cols, &flat_matrix)
    }

    // Функция f(t, x), возвращающая производную состояния
    fn f(_t: f64, state: &DVector<f64>, matrix: &DMatrix<f64>) -> DVector<f64> {
        matrix * state
    }

    // Метод Рунге-Кутты 4-го порядка для одного шага
    fn runge_kutta4_step(&self, state: &DVector<f64>, matrix: &DMatrix<f64>, t: f64, dt: f64) -> DVector<f64> {
        let k1 = Self::f(t, state, matrix);
        let k2 = Self::f(t + dt / 2.0, &(state + &k1 * (dt / 2.0)), matrix);
        let k3 = Self::f(t + dt / 2.0, &(state + &k2 * (dt / 2.0)), matrix);
        let k4 = Self::f(t + dt, &(state + &k3 * dt), matrix);

        let new_state = state + &k1 * (dt / 6.0) + &k2 * (dt / 3.0) + &k3 * (dt / 3.0) + &k4 * (dt / 6.0);

        // Нормализация нового состояния
        let sum: f64 = new_state.iter().sum();
        new_state / sum
    }


    // Интегрирование системы уравнений
    pub fn integrate_system(&self) -> Vec<DVector<f64>> {
        let matrix = Self::kolmogorov_matrix_to_dmatrix(self.generate_kolmogorov_matrix());
        let initial_state_vec = Self::initial_state_to_dvector(Arc::clone(&QUEUING_SYSTEM_CONFIG.initial_state));
        let delta_t = QUEUING_SYSTEM_CONFIG.step_size;

        std::iter::successors(Some((initial_state_vec, 0.0)), |(last_state, t)| {
            Some((self.runge_kutta4_step(last_state, &matrix, *t, delta_t), t + delta_t))
        })
            .take((QUEUING_SYSTEM_CONFIG.num_iterations + 1) as usize)
            .map(|(state, _)| state)
            .collect()
    }


    pub fn plot_states(&self, states: Vec<DVector<f64>>) -> Result<(), Box<dyn std::error::Error>> {
        let root_area = BitMapBackend::new("channels_states.png", (1024, 768)).into_drawing_area();
        root_area.fill(&WHITE)?;

        let num_states = states.first().map_or(0, |v| v.len());
        let num_steps = states.len();

        let max_y = states.iter().flatten().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_y = states.iter().flatten().cloned().fold(f64::INFINITY, f64::min);

        let mut chart = ChartBuilder::on(&root_area)
            .caption("System States Over Time", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..num_steps, min_y..max_y)?;

        chart.configure_mesh().draw()?;

        let colors = [
            &RED, &GREEN, &BLUE, &YELLOW, &CYAN, &MAGENTA, &BLACK,
            // Дополнительные цвета, если у вас больше состояний
        ];

        let mut series = Vec::new();

        for i in 0..num_states {
            let line_series = LineSeries::new(
                states.iter().enumerate().map(|(step, state)| (step, state[i])),
                colors[i % colors.len()],
            );
            series.push((line_series, format!("z_{}", i + 1), colors[i % colors.len()]));
        }

        for (line_series, label, &color) in series {
            chart
                .draw_series(line_series)
                .expect("Failed to draw line series")
                .label(label)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root_area.present()?;
        Ok(())
    }

}










