
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
        SMO_CONFIG.time
    );
    //smo.plot_state_graph().expect("Failed to plot state graph");

    let transition_matrix = smo.generate_kolmogorov_matrix();
    println!("{:?}", transition_matrix);
}
