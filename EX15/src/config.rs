use thiserror::Error;

#[derive(Debug)]
pub struct Config {
    num_channels: i32,
    queue_size: i32,
    lambda_rate: i32,
    mu_rate: i32,
    initial_state: [i32; 7],
    time: i32
}

pub const SMO_CONFIG: Config = Config {
    num_channels: 3,
    queue_size: 3,
    lambda_rate: 30,
    mu_rate: 5,
    initial_state: [1, 0, 0, 0, 0, 0, 0],
    time: 1
};

pub enum SmoError {

}