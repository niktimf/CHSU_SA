

use plotters::prelude::*;
use crate::config::SmoError;

pub struct SMO {
    lambda: f64, // Интенсивность потока заявок
    mu: f64,     // Интенсивность обработки одним офицером
    num_channels: usize, // Количество офицеров
    queue_size: usize, // Ограничение на размер очереди
}

impl SMO {
    pub fn new(lambda: f64, mu: f64, num_channels: usize, queue_size: usize) -> SMO {
        SMO {
            lambda,
            mu,
            num_channels,
            queue_size,
        }
    }

    pub fn plot_state_graph(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dimensions = (1024, 768);
        let root_area = BitMapBackend::new("smo_states.png", dimensions).into_drawing_area();
        root_area.fill(&WHITE)?;


        let states = self.num_channels + self.queue_size + 1;
        let step_x = dimensions.0 as f32 / states as f32;
        let step_y = dimensions.1 as f32 / 2.0;

        for i in 0..states {
            // Draw rectangles for states
            let x = i as f32 * step_x;
            root_area.draw(&Rectangle::new(
                [(x as i32, (step_y - 30.0) as i32), ((x + step_x - 10.0) as i32, (step_y + 30.0) as i32)],
                Into::<ShapeStyle>::into(&BLACK).filled(),
            ))?;

            // State labels
            let text_style = TextStyle::from(("sans-serif", 20).into_font()).color(&BLACK);
            root_area.draw_text(
                &format!("S_{}", i),
                &text_style,
                ((x + step_x / 2.0 - 15.0) as i32, step_y as i32),
            )?;

            if i > 0 {
                // Lambda arrows
                root_area.draw(&PathElement::new(
                    vec![(x - step_x / 2.0, step_y - 40.0), (x - 10.0, step_y - 40.0)]
                        .iter().map(|&(x, y)| (x as i32, y as i32)).collect::<Vec<_>>(),
                    ShapeStyle {
                        color: BLUE.to_rgba(),
                        filled: false,
                        stroke_width: 2,
                    },
                ))?;

                root_area.draw_text(
                    &format!("λ = {}", self.lambda),
                    &text_style.color(&BLUE),
                    ((x - step_x / 2.0) as i32, (step_y - 70.0) as i32),
                )?;

                // Mu arrows
                let mu_value = i as f64 * self.mu;
                root_area.draw(&PathElement::new(
                    vec![(x - 10.0, step_y + 40.0), (x - step_x / 2.0, step_y + 40.0)]
                        .iter().map(|&(x, y)| (x as i32, y as i32)).collect::<Vec<_>>(),
                    ShapeStyle {
                        color: RED.to_rgba(),
                        filled: false,
                        stroke_width: 2,
                    },
                ))?;
                root_area.draw_text(
                    &format!("μ = {:.1}", mu_value),
                    &text_style.color(&RED),
                    ((x - step_x / 2.0) as i32, (step_y + 70.0) as i32),
                )?;
            }
        }

        root_area.present()?;

        Ok(())
    }
}










