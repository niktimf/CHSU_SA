use std::collections::BTreeMap;
use crate::queuing_system::QueuingSystem;

pub trait QueuingSystemCharacteristics {
    fn calculate_load_factor(&self) -> f64;
    fn calculate_probability_of_downtime(&self) -> f64;
    fn factorial(n: u64) -> u64;
    fn calculate_probabilities(&self) -> BTreeMap<String, f64>;
    fn calculate_queue_probabilities(&self) -> BTreeMap<String, f64>;
    fn calculate_rejection_probability(&self) -> f64;
    fn calculate_average_incoming_requests_during_t(&self) -> i32;
    fn calculate_average_service_time_per_request(&self) -> f64;
    fn average_service_time_per_channel_for_t(&self) -> f64;
    fn calculate_average_busy_channels(&self) -> f64;
    fn calculate_average_number_of_requests_in_queue(&self) -> f64;
    fn calculate_average_waiting_time_in_queue(&self) -> f64;
    fn calculate_total_number_of_requests(&self) -> f64;
    fn calculate_average_waiting_time(&self) -> f64;
    fn calculate_average_time_in_system(&self) -> f64;
}


impl QueuingSystemCharacteristics for QueuingSystem {
    /// 1
    /// Вычисляет коэффициент загрузки СМО.
    /// # Возвращаемое значение
    /// Коэффициент загрузки системы СМО, тип: `f64`.
    fn calculate_load_factor(&self) -> f64 {
        self.lambda_rate as f64 / self.mu_rate as f64
    }


    /// 2
    /// Вычисляет вероятность простоя системы (P0).
    /// # Возвращаемое значение
    /// Вероятность простоя системы, тип: `f64`.
    fn calculate_probability_of_downtime(&self) -> f64 {
        let ksi = self.calculate_load_factor();
        let sum1: f64 = (0..self.num_channels)
            .map(|i| ksi.powi(i) / Self::factorial(i as u64) as f64)
            .sum();
        let sum2: f64 = (self.num_channels..=(self.num_channels + self.queue_size))
            .map(|i| (self.num_channels.pow(self.num_channels as u32) as f64 / Self::factorial(self.num_channels as u64) as f64)
                * (ksi / self.num_channels as f64).powi(i))
            .sum();
        1.0 / (sum1 + sum2)
    }

    /// Вычисляет факториал числа.
    /// # Параметры
    /// * `n` - Число, для которого вычисляется факториал.
    /// # Возвращаемое значение
    /// Факториал заданного числа, тип: `u64`.
    fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }

    /// 3
    /// Вероятность того, что i каналов заняты и нет очереди.
    /// # Возвращаемое значение
    /// Ключ(состояние системы) и значение(вероятность этого состояния), тип: `BTreeMap<String, f64>`.
    fn calculate_probabilities(&self) -> BTreeMap<String, f64> {
        let p0 = self.calculate_probability_of_downtime();
        let ksi = self.calculate_load_factor();

        (0..=self.num_channels)
            .map(|i| (format!("P_{}", i), p0 * ksi.powi(i) / Self::factorial(i as u64) as f64))
            .collect()
    }

    /// 4
    /// Вероятность того, что все s каналов заняты и очередь длины i.
    /// # Возвращаемое значение
    /// Ключ(состояние системы с очередью) и значение(вероятность этого состояния), тип: `BTreeMap<String, f64>`.
    fn calculate_queue_probabilities(&self) -> BTreeMap<String, f64> {
        let p0 = self.calculate_probability_of_downtime();
        let ksi = self.calculate_load_factor();
        let s = self.num_channels as f64;
        let n = self.queue_size;

        (1..=n).map(|i| {
            let pi = p0 * (s.powf(s) / Self::factorial(s as u64) as f64) * (ksi / s).powi(i + s as i32);
            (format!("P{}", i), pi)
        }).collect()
    }

    /// 5
    /// Вероятность отказа не попасть в очередь длины n, все каналы заняты и очередь уже сформирована.
    /// # Возвращаемое значение
    /// Вероятность отказа, когда все каналы заняты и очередь достигла максимальной длины, тип: `f64`.
    fn calculate_rejection_probability(&self) -> f64 {
        let p0 = self.calculate_probability_of_downtime();
        let ksi = self.calculate_load_factor();
        let s = self.num_channels as f64;
        let n = self.queue_size;

        p0 * (s.powf(s) / Self::factorial(s as u64) as f64) * (ksi / s).powi(n + s as i32)
    }

    /// 6
    /// Вычисляет среднее количество заявок, поступающих в систему за время T.
    /// # Возвращаемое значение
    /// Среднее количество заявок за указанный период времени, тип: `i32`.
    fn calculate_average_incoming_requests_during_t(&self) -> i32 {
        self.lambda_rate * self.time
    }

    /// 7
    /// Вычисляет среднее время обслуживания одной заявки.
    /// # Возвращаемое значение
    /// Среднее время, необходимое для обслуживания одной заявки, тип: `f64`.
    fn calculate_average_service_time_per_request(&self) -> f64 {
        1.0 / self.mu_rate as f64
    }

    /// 8
    /// Вычисляет среднее время обслуживания одним каналом заявок, поступивших за время T.
    /// # Возвращаемое значение
    /// Среднее время обслуживания заявок одним каналом за время T, тип: `f64`.
    fn average_service_time_per_channel_for_t(&self) -> f64 {
        let ksi = self.calculate_load_factor();
        ksi * self.time as f64
    }

    /// 9
    /// Вычисляет среднее число занятых каналов в системе.
    /// # Возвращаемое значение
    /// Среднее количество занятых каналов в системе, тип: `f64`.
    fn calculate_average_busy_channels(&self) -> f64 {
        let probabilities = self.calculate_probabilities();
        let queue_probabilities = self.calculate_queue_probabilities();

        probabilities.iter()
            .chain(queue_probabilities.iter())
            .fold(0.0, |acc, (key, prob)| {
                let channel_count = key.strip_prefix("P")
                    .and_then(|num| num.parse::<usize>().ok())
                    .unwrap_or(0);
                acc + channel_count as f64 * prob
            })
    }

    /// 10
    /// Вычисляет среднее количество заявок в очереди.
    /// # Возвращаемое значение
    /// Среднее количество заявок в очереди, тип: `f64`.
    fn calculate_average_number_of_requests_in_queue(&self) -> f64 {
        let ksi = self.calculate_load_factor();
        let p0 = self.calculate_probability_of_downtime();
        let s = self.num_channels as f64;

        let numerator = ksi.powf(s + 1.0);
        let denominator = Self::factorial((s - 1.0) as u64) as f64 * (s - ksi).powi(2);

        (numerator / denominator) * p0
    }

    /// 12
    /// Вычисляет среднее время пребывания заявки в очереди.
    /// # Возвращаемое значение
    /// Среднее время пребывания заявки в очереди, тип: `f64`.
    fn calculate_average_waiting_time_in_queue(&self) -> f64 {
        let average_number_of_requests_in_queue = self.calculate_average_number_of_requests_in_queue();
        let lambda = self.lambda_rate as f64;

        average_number_of_requests_in_queue / lambda
    }

    /// 13
    /// Вычисляет общее количество заявок в системе.
    /// # Возвращаемое значение
    /// Общее количество заявок в системе, тип: `f64`.
    fn calculate_total_number_of_requests(&self) -> f64 {
        let average_number_of_requests_in_queue = self.calculate_average_number_of_requests_in_queue();
        let ksi = self.calculate_load_factor();

        average_number_of_requests_in_queue + ksi
    }

    /// 14
    /// Вычисляет среднее время ожидания заявки в системе.
    /// # Возвращаемое значение
    /// Среднее время ожидания заявки в системе, тип: `f64`.
    fn calculate_average_waiting_time(&self) -> f64 {
        let average_number_of_requests_in_queue = self.calculate_average_number_of_requests_in_queue();
        let lambda = self.lambda_rate as f64;

        average_number_of_requests_in_queue / lambda
    }

    /// 15
    /// Вычисляет среднее время пребывания заявки в системе.
    /// # Возвращаемое значение
    /// Среднее время пребывания заявки в системе, тип: `f64`.
    fn calculate_average_time_in_system(&self) -> f64 {
        let total_number_of_requests = self.calculate_total_number_of_requests();
        let lambda = self.lambda_rate as f64;

        total_number_of_requests / lambda
    }



}

