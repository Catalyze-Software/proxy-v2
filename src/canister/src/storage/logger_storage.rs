use super::{
    storage_api::{LOGS, LOGS_INDEX},
    StorageMethods,
};
use canister_types::models::{
    api_error::ApiError,
    logger::{Log, Logger, PostLog},
};

pub struct LoggerStore;
pub struct LogsIndex;

pub const NAME: &str = "logs";
pub const MAX_LOGS: u64 = 10_000;

impl LoggerStore {
    /// Create a new logger from a log
    /// # Arguments
    /// * `log` - The log to create the logger from
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if created, otherwise an error
    fn new(log: Log) -> Result<(u64, Logger), ApiError> {
        let id = LogsIndex::new();
        let logger = Logger::from_log(id, log);

        Self::insert(logger)
    }

    /// Create a new logger from a post log
    /// # Arguments
    /// * `post_log` - The post log to create the logger from
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if created, otherwise an error
    pub fn new_from_post_log(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
        let log = Log::from_post_log(post_log);
        Self::new(log)
    }

    /// Create a new logger from a post log with the caller
    /// # Arguments
    /// * `post_log` - The post log to create the logger from
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if created, otherwise an error
    pub fn new_from_post_log_with_caller(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
        let log = Log::from_post_log_with_caller(post_log);
        Self::new(log)
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
    fn get_many(keys: Vec<u64>) -> Vec<(u64, Logger)> {
        LOGS.with(|logs| {
            let mut logs_vec = Vec::new();
            for id in keys {
                if let Some(log) = logs.borrow().get(&id) {
                    logs_vec.push((id, log.clone()));
                }
            }
            logs_vec
        })
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

    /// Insert a logger based on the id modulo the max logs
    /// # Arguments
    /// * `logger` - The logger to insert
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if inserted, otherwise an error
    fn insert(logger: Logger) -> Result<(u64, Logger), ApiError> {
        LOGS.with(|logs| {
            let id = logger.id;

            // Base the index on the id modulo the max logs
            let index = id % MAX_LOGS;

            logs.borrow_mut().insert(index, logger.clone());
            Ok((index, logger))
        })
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

impl LogsIndex {
    /// Inrement the logs index and return the new index
    /// # Returns
    /// * `u64` - The new logs index
    fn new() -> u64 {
        LOGS_INDEX.with(|index| {
            let mut cell = index.borrow_mut();
            let current_index = cell.get();
            let new_index = current_index + 1;

            cell.set(new_index).expect("Failed to increment logs index");

            new_index
        })
    }
}
