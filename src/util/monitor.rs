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
        self.is_ready.store(true, Ordering::Release);
    }
}

impl Drop for ReadyGuard {
    fn drop(&mut self) {
        self.is_ready.store(true, Ordering::Release);
    }
}

/// Run an action in another thread if the ReadyGuard has not dropped after the timeout.
pub fn run_if_not_ready_after(timeout: Duration, timeout_action: impl FnOnce() + Send + 'static) -> ReadyGuard {
    let is_ready = Arc::new(AtomicBool::new(false));
    let is_ready_clone = is_ready.clone();
    thread::spawn(move || {
        sleep(timeout);
        if is_ready_clone.load(Ordering::Acquire) {
            debug!("operation was ready after {} ms, not running action", timeout.as_millis());
            return;
        }
        debug!("operation not ready after {} ms, running action", timeout.as_millis());
        timeout_action();
    });
    ReadyGuard { is_ready }
}

#[test]
fn with_timeout() {
    let is_invoked = Arc::new(AtomicBool::new(false));
    let is_invoked_clone = is_invoked.clone();
    run_if_not_ready_after(Duration::from_millis(1), move || {
        is_invoked_clone.store(true, Ordering::Release);
    });
    for i in 0 .. 200 {
        sleep(Duration::from_millis(1));
        if is_invoked.load(Ordering::Acquire) {
            return
        }
    }
    panic!("should have invoked the action and early-exited")
}

#[test]
fn no_timeout() {
    {
        run_if_not_ready_after(Duration::from_millis(1), || {
            panic!("should not have run");
        });
    }
    sleep(Duration::from_millis(2));
}
