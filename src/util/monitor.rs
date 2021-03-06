use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering;
use ::std::sync::Arc;
use ::std::thread;
use ::std::thread::sleep;
use ::std::time::Duration;

use ::log::debug;

//TODO @mark: use

#[derive(Debug)]
pub struct ReadyGuard {
    #[allow(unused)] //TODO @mark:
    is_ready: Arc<AtomicBool>,
}

impl ReadyGuard {
    #[allow(unused)] //TODO @mark:
    pub fn ready(&self) {
        self.is_ready.store(true, Ordering::Release);
    }
}

/// Implementing Drop means that triggering non-lexical early drop never runs the action,
/// so it is better to not drop and trigger the action too eagerly if .ready() is not called,
/// since that is probably  easier to notice in testing.
// impl Drop for ReadyGuard {
//     fn drop(&mut self) {
//         self.is_ready.store(true, Ordering::Release);
//     }
// }

/// Run an action in another thread if the ReadyGuard has not dropped after the timeout.
#[allow(unused)] //TODO @mark:
pub fn run_if_not_ready_after(
    timeout: Duration,
    timeout_action: impl FnOnce() + Send + 'static,
) -> ReadyGuard {
    let is_ready = Arc::new(AtomicBool::new(false));
    let is_ready_clone = is_ready.clone();
    thread::spawn(move || {
        sleep(timeout);
        if is_ready_clone.load(Ordering::Acquire) {
            debug!(
                "operation was ready after {} ms, not running action",
                timeout.as_millis()
            );
            return;
        }
        debug!(
            "operation not ready after {} ms, running action",
            timeout.as_millis()
        );
        timeout_action();
    });
    ReadyGuard { is_ready }
}

#[test]
fn with_timeout() {
    let is_invoked = Arc::new(AtomicBool::new(false));
    let is_invoked_clone = is_invoked.clone();
    let guard = run_if_not_ready_after(Duration::from_millis(1), move || {
        is_invoked_clone.store(true, Ordering::Release);
    });
    for _ in 0..20 {
        sleep(Duration::from_millis(1));
        if is_invoked.load(Ordering::Acquire) {
            return;
        }
    }
    drop(guard); // explicit drop actually extends the lifetime to here
    panic!("should have invoked the action and early-exited")
}

#[test]
fn no_timeout() {
    {
        let guard = run_if_not_ready_after(Duration::from_millis(1), || {
            panic!("should not have run");
        });
        drop(guard); // explicit drop actually extends the lifetime to here
    }
    sleep(Duration::from_millis(2));
}
