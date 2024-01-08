
use std::sync::Arc;
use crate::config::QUEUING_SYSTEM_CONFIG;
use crate::queuing_system::QueuingSystem;
use crate::queuing_system_characteristics::QueuingSystemCharacteristics;

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
    println!("Правые части уравнений Колмогорова: {:?}", matrix);

    let states = queuing_system.integrate_system();
    println!("{:#?}", states);

    //queuing_system.plot_states(states).expect("Failed to plot states");

    let load_factor = queuing_system.calculate_load_factor();
    let probability_of_downtime = queuing_system.calculate_probability_of_downtime();
    let probabilities = queuing_system.calculate_probabilities();
    let queue_probabilities = queuing_system.calculate_queue_probabilities();
    let rejection_probability = queuing_system.calculate_rejection_probability();
    let average_incoming_requests_during_t = queuing_system.calculate_average_incoming_requests_during_t();
    let average_service_time_per_request = queuing_system.calculate_average_service_time_per_request();
    let average_busy_channels = queuing_system.calculate_average_busy_channels();
    let average_number_of_requests_in_queue = queuing_system.calculate_average_number_of_requests_in_queue();
    let average_waiting_time_in_queue = queuing_system.calculate_average_waiting_time_in_queue();
    let total_number_of_requests = queuing_system.calculate_total_number_of_requests();
    let average_waiting_time = queuing_system.calculate_average_waiting_time();
    let average_time_in_system = queuing_system.calculate_average_time_in_system();


    println!("Коэффициент загрузки СМО: {}", load_factor);
    println!("Вероятность простоя системы: {}", probability_of_downtime);
    println!("Вероятности того, что i  каналов заняты и нет очереди: {:?}", probabilities);
    println!("Вероятности того, что все s каналов заняты и очередь длины i: {:?}", queue_probabilities);
    println!("Вероятность отказа не попасть в очередь длины n, все каналы заняты и очередь уже сформирована: {:?}", rejection_probability);
    println!("Среднее число заявок, поступающих за время T: {}", average_incoming_requests_during_t);
    println!("Среднее время обслуживания заявки: {}", average_service_time_per_request);
    println!("Среднее число занятых каналов: {}", average_busy_channels);
    println!("Среднее число заявок в очереди: {}", average_number_of_requests_in_queue);
    println!("Среднее время пребывания заявки в очереди: {}", average_waiting_time_in_queue);
    println!("Общее количество заявок в системе: {}", total_number_of_requests);
    println!("Среднее время ожидания заявки в системе: {}", average_waiting_time);
    println!("Среднее время пребывания заявки в системе: {}", average_time_in_system);
}
