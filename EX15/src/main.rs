use crate::smo::SMO;

mod config;
mod smo;
mod smo_characteristics;


fn main() {
    let smo = SMO::new(30.0, 5.0, 3, 3);
    //smo.plot_state_graph().expect("Failed to plot state graph");
}
