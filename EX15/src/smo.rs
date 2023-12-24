use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use plotters::coord::Shift;
use plotters::prelude::*;
use crate::config::SmoError;

pub struct SMO {
    lambda: i32, // Интенсивность потока заявок
    mu: i32,     // Интенсивность обработки одним офицером
    num_channels: i32, // Количество офицеров
    queue_size: i32, // Ограничение на размер очереди
    initial_state: Arc<HashMap<&'static str, i32>>,
}

impl SMO {
    pub fn new(lambda: i32,
               mu: i32,
               num_channels: i32,
               queue_size: i32,
               initial_state: Arc<HashMap<&'static str, i32>>
    ) -> SMO {

        SMO {
            lambda,
            mu,
            num_channels,
            queue_size,
            initial_state,
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

}










