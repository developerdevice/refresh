use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::errors::{RefreshError, Result};

/// Global flag to track if SIGINT (Ctrl+C) was received
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// Setup signal handler for SIGINT (Ctrl+C)
pub fn setup_signal_handler() -> Result<Arc<AtomicBool>> {
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();

    ctrlc::set_handler(move || {
        INTERRUPTED.store(true, Ordering::SeqCst);
        interrupted_clone.store(true, Ordering::SeqCst);
    })
    .map_err(|e| RefreshError::Signal(format!("Failed to set signal handler: {}", e)))?;

    Ok(interrupted)
}

/// Check if the program was interrupted by SIGINT
pub fn is_interrupted(interrupted: &Arc<AtomicBool>) -> bool {
    interrupted.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interrupted_flag_initial_state() {
        let interrupted = Arc::new(AtomicBool::new(false));
        assert!(!is_interrupted(&interrupted));
    }

    #[test]
    fn test_interrupted_flag_set() {
        let interrupted = Arc::new(AtomicBool::new(false));
        interrupted.store(true, Ordering::SeqCst);
        assert!(is_interrupted(&interrupted));
    }
}
