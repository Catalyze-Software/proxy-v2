use super::{storage_api::LOGS, StorageMethods};
use canister_types::models::{
    api_error::ApiError,
    logger::{Logger, PostLog},
};

pub struct LoggerStore;

pub const NAME: &str = "logs";
pub const MAX_LOGS: u64 = 5;

impl LoggerStore {
    /// Create a new logger from a post log
    /// # Arguments
    /// * `post_log` - The post log to create the logger from
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if created, otherwise an error
    pub fn new_from_post_log(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
        let log = Logger::from_post_log(post_log);
        Self::insert(log)
    }

    /// Create a new logger from a post log with the caller
    /// # Arguments
    /// * `post_log` - The post log to create the logger from
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if created, otherwise an error
    pub fn new_from_post_log_with_caller(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
        let log = Logger::from_post_log_with_caller(post_log);
        Self::insert(log)
    }

    pub fn size() -> u64 {
        LOGS.with(|logs| logs.borrow().len() as u64)
    }

    pub fn new_key() -> u64 {
        LOGS.with(|logs| match logs.borrow().last_key_value() {
            Some((key, _)) => key + 1,
            None => 1,
        })
    }

    /// Get the latest logs from most recent to oldest
    /// # Arguments
    /// * `amount` - The number of logs to get
    /// # Returns
    /// * `Result<Vec<(u64, Logger)>, ApiError>` - The logs if found, otherwise an error
    pub fn get_latest_logs(amount: u64) -> Result<Vec<Logger>, ApiError> {
        let last_key = LOGS.with(|logs| logs.borrow().last_key_value().expect("empty logs").0);

        if amount > last_key {
            return Err(ApiError::bad_request()
                .add_method_name("get_latest_logs: amount exceeds log size")
                .add_info(NAME));
        }

        if amount == 0 {
            return Ok(Vec::new());
        }

        if amount > MAX_LOGS {
            return Err(ApiError::bad_request()
                .add_method_name("get_latest_logs: amount exceeds max logs")
                .add_info(NAME));
        }

        LOGS.with(|logs| {
            let logs = logs.borrow();

            let mut logs_vec = Vec::new();

            let range = (last_key - amount + 1)..=last_key;

            for (_, value) in logs.range(range) {
                logs_vec.push(value.clone());
            }
            logs_vec.reverse();

            Ok(logs_vec)
        })
    }
}

impl StorageMethods<u64, Logger> for LoggerStore {
    /// Get a single logger by id
    /// # Arguments
    /// * `key` - The key of the report to get
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if found, otherwise an error
    fn get(key: u64) -> Result<(u64, Logger), ApiError> {
        LOGS.with(|logs| {
            logs.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|log| (key, log.clone()))
        })
    }

    /// Get multiple loggers by keys
    /// # Arguments
    /// * `keys` - The keys of the loggers to get
    /// # Returns
    /// * `Vec<(u64, Logger)>` - The loggers (and their keys) if found, otherwise an empty vector
    fn get_many(_: Vec<u64>) -> Vec<(u64, Logger)> {
        // LOGS.with(|logs| {
        //     let mut logs_vec = Vec::new();
        //     for id in keys {
        //         if let Some(log) = logs.borrow().get(&id) {
        //             logs_vec.push((id, log.clone()));
        //         }
        //     }
        //     logs_vec
        // })
        todo!()
    }

    /// Find a single logger by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, Logger)>` - The logger if found, otherwise None
    fn find<F>(filter: F) -> Option<(u64, Logger)>
    where
        F: Fn(&u64, &Logger) -> bool,
    {
        LOGS.with(|logs| logs.borrow().iter().find(|(id, log)| filter(id, log)))
    }

    /// Find all loggers by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, Logger)>` - The loggers if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(u64, Logger)>
    where
        F: Fn(&u64, &Logger) -> bool,
    {
        LOGS.with(|logs| {
            logs.borrow()
                .iter()
                .filter(|(id, log)| filter(id, log))
                .collect()
        })
    }

    /// # Arguments
    /// * `logger` - The logger to insert
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if inserted, otherwise an error
    fn insert(logger: Logger) -> Result<(u64, Logger), ApiError> {
        let key = Self::new_key();

        LOGS.with(|logs| logs.borrow_mut().insert(key, logger.clone()));

        while Self::size() > MAX_LOGS {
            LOGS.with(|logs| {
                let mut logs = logs.borrow_mut();
                let first_key_val = logs
                    .first_key_value()
                    .expect("Failed to get first key value");
                logs.remove(&first_key_val.0);
            });
        }

        Ok((key, logger))
    }

    fn insert_by_key(_: u64, _: Logger) -> Result<(u64, Logger), ApiError> {
        todo!()
    }

    fn update(_: u64, _: Logger) -> Result<(u64, Logger), ApiError> {
        todo!()
    }

    fn remove(_: u64) -> bool {
        todo!()
    }
}
