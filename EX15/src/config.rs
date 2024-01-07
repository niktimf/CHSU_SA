
use lazy_static::lazy_static;
use std::sync::Arc;

#[derive(Debug)]
pub struct Config {
    pub num_channels: i32,
    pub queue_size: i32,
    pub lambda_rate: i32,
    pub mu_rate: i32,
    pub initial_state: Arc<Vec<(&'static str, i32)>>,
    pub time: i32,
    pub num_iterations: i32,
    pub step_size: f64
}

/// Инициализация начальных даанных из условий задачи
lazy_static! {
    pub static ref QUEUING_SYSTEM_CONFIG: Config = Config {
        num_channels: 3,
        queue_size: 3,
        lambda_rate: 30,
        mu_rate: 5,
        initial_state: Arc::new(vec![
            ("S_0", 1),
            ("S_1", 0),
            ("S_2", 0),
            ("S_3", 0),
            ("S_4", 0),
            ("S_5", 0),
            ("S_6", 0),
        ]),
        time: 1,
        num_iterations: 100,
        step_size: 0.01
    };
}
