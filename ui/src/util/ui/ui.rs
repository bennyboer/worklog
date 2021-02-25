//! Utility methods for the UI.

use druid::Monitor;

/// Get the primary monitor from the given list of monitors.
pub fn get_primary_monitor(monitors: &[Monitor]) -> Option<&Monitor> {
    for monitor in monitors {
        if monitor.is_primary() {
            return Some(monitor);
        }
    }

    None
}
