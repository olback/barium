use {
    tokio::runtime,
    crate::{error::BariumResult, config::Config, is_debug},
    log::trace
};

pub struct TokioRuntimeBuilder(runtime::Builder);

impl TokioRuntimeBuilder {

    pub fn new() -> Self {

        let mut inner = runtime::Builder::new();
        inner.thread_name("barium-thread-pool");
        inner.threaded_scheduler();
        inner.enable_io();
        inner.enable_time();

        if is_debug!() {
            inner.on_thread_start(|| trace!("Starting native thread"));
            inner.on_thread_stop(|| trace!("Stopping native thread"));
        }

        Self(inner)

    }

    pub fn from_config(conf: &Config) -> Self {

        let mut inner = Self::new();

        if let Some(ref core_threads) = conf.runtime.core_threads {
            inner.0.core_threads(*core_threads);
        }

        if let Some(ref max_threads) = conf.runtime.max_threads {
            inner.0.max_threads(*max_threads);
        }

        inner

    }

    pub fn build(mut self) -> BariumResult<runtime::Runtime> {
        Ok(self.0.build()?)
    }

}
