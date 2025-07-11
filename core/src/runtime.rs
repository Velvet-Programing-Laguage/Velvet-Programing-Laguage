use crate::config::Config;
use crate::logger::Logger;
use rayon::ThreadPoolBuilder;

pub struct Runtime {
    config: Config,
    logger: Logger,
    thread_pool: rayon::ThreadPool,
}

impl Runtime {
    /// Tworzy nowy obiekt Runtime, w tym pulę wątków.
    /// Jeśli w konfiguracji liczba wątków wynosi 0, ustawia 1.
    pub fn new(config: Config) -> Self {
        let thread_count = if config.runtime.max_threads == 0 {
            1 // minimalna liczba wątków
        } else {
            config.runtime.max_threads
        };
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .build()
            .unwrap_or_else(|e| {
                panic!("Failed to build thread pool: {}", e);
            });
        Runtime {
            config,
            logger: Logger::new(config.debug),
            thread_pool,
        }
    }

    /// Inicjuje środowisko wykonawcze i loguje liczbę wątków.
    pub fn init(&self, args: &str) -> String {
        self.logger
            .info(&format!(
                "Runtime initialized with {} threads",
                self.config.runtime.max_threads
            ));
        format!("Runtime initialized with args: {}", args)
    }

    /// Wykonuje funkcję równolegle w puli wątków.
    pub fn execute_parallel<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R + Send,
        R: Send,
    {
        self.thread_pool.install(f)
    }
}
