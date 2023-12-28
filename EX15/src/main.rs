
use std::sync::Arc;
use crate::config::SMO_CONFIG;
use crate::smo::SMO;

mod config;
mod smo;
mod smo_characteristics;


fn main() {

    let smo = SMO::new(
        SMO_CONFIG.lambda_rate,
        SMO_CONFIG.mu_rate,
        SMO_CONFIG.num_channels,
        SMO_CONFIG.queue_size,
        Arc::clone(&SMO_CONFIG.initial_state),
        SMO_CONFIG.time,
        SMO_CONFIG.num_iterations,
        SMO_CONFIG.step_size
    );
    //smo.plot_state_graph().expect("Failed to plot state graph");

    let transition_matrix = smo.generate_kolmogorov_matrix();
    //println!("{:?}", transition_matrix);


    let f_tx = smo.multiply_matrix_vector(
        transition_matrix,
        Arc::clone(&SMO_CONFIG.initial_state)
    );

    println!("{:?}", f_tx);

    let states = smo.integrate_system();
    //println!("{:?}", states);

    //smo.plot_states(states).expect("Failed to plot states");
}
