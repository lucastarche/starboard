use std::sync::Mutex;

/// Extension trait for Rust's standard Mutex type
pub trait MutexExt<T>: sealed::Sealed {
    /// Locks the Mutex and gets us a MutexGuard even if the Mutex is poisoned
    fn locked(&self) -> std::sync::MutexGuard<T>;
}

impl<T> MutexExt<T> for Mutex<T> {
    fn locked(&self) -> std::sync::MutexGuard<T> {
        self.lock().unwrap_or_else(|poison| poison.into_inner())
    }
}

mod sealed {
    pub trait Sealed {}
}

impl<T> sealed::Sealed for Mutex<T> {}
