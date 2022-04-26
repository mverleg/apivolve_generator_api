use ::std::io::Read;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering;
use ::std::thread;
use ::std::thread::sleep;
use ::std::time::Duration;

use ::log::debug;

#[derive(Debug)]
pub struct ReadyGuard {
    is_ready: Arc<AtomicBool>,
}

impl ReadyGuard {
    pub fn ready(&self) {
        self.is_ready.store(true, Ordering::Acquire);
    }
}

impl Drop for ReadyGuard {
    fn drop(&mut self) {
        self.is_ready.store(true, Ordering::Release);
    }
}

/// Run an action in another thread if the ReadyGuard has not dropped after the timeout.
#[must_use]
pub fn run_if_not_ready_after(timeout: Duration, timeout_action: impl FnOnce()) -> ReadyGuard {
    let is_ready = Arc::new(AtomicBool::new(false));
    thread::spawn(move || {
        sleep(timeout);
        if is_ready.load(Ordering::Acquire) {
            debug!("operation was ready after {} ms, not running action", timeout.as_millis());
            return;
        }
        debug!("operation not ready after {} ms, running action", timeout.as_millis());
        timeout_action();
    });
    ReadyGuard { is_ready }
}

