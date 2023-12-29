
use std::sync::Arc;
use crate::config::QUEUING_SYSTEM_CONFIG;
use crate::queuing_system::QueuingSystem;

mod config;
mod queuing_system;
mod queuing_system_characteristics;


fn main() {

    let queuing_system = QueuingSystem::new(
        QUEUING_SYSTEM_CONFIG.lambda_rate,
        QUEUING_SYSTEM_CONFIG.mu_rate,
        QUEUING_SYSTEM_CONFIG.num_channels,
        QUEUING_SYSTEM_CONFIG.queue_size,
        Arc::clone(&QUEUING_SYSTEM_CONFIG.initial_state),
        QUEUING_SYSTEM_CONFIG.time,
        QUEUING_SYSTEM_CONFIG.num_iterations,
        QUEUING_SYSTEM_CONFIG.step_size
    );
    //queuing_system.plot_state_graph().expect("Failed to plot state graph");

    let matrix = queuing_system.generate_kolmogorov_matrix();
    //println!("{:?}", matrix);

    let states = queuing_system.integrate_system();
    //println!("{:#?}", states);

    queuing_system.plot_states(states).expect("Failed to plot states");
}
