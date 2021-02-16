/// Time events.
pub struct TimeEvent {
    /// Whether the timer is started.
    pub is_start: bool,
    /// Timestamp of the event.
    pub timestamp: i64,
}

impl TimeEvent {
    pub fn new(is_start: bool, timestamp: i64) -> TimeEvent {
        TimeEvent {
            is_start,
            timestamp,
        }
    }
}

/// Calculate the total work time without counting parallel work time ranges
/// multiple times.
/// For example when you have parallel work items, the time is only counted once!
pub fn calculate_unique_total_time(events: &mut [TimeEvent]) -> i64 {
    // Sort events by their timestamp
    events.sort_by_key(|v| v.timestamp);

    // Iterate over events and sum up the total time
    let mut total_time = 0;
    let mut in_progress_count = 0;
    let mut start_timestamp: Option<i64> = None;
    for event in events {
        if event.is_start {
            in_progress_count += 1;

            if in_progress_count == 1 {
                start_timestamp = Some(event.timestamp); // Memorize as start timestamp
            }
        } else {
            in_progress_count -= 1;

            if in_progress_count == 0 {
                // No time range currently in progress -> save to time taken
                total_time += event.timestamp
                    - start_timestamp.expect("Start timestamp must be defined here!");
            }
        }
    }

    total_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial() {
        let result =
            calculate_unique_total_time(&mut [TimeEvent::new(true, 10), TimeEvent::new(false, 20)]);

        assert_eq!(result, 10);
    }

    #[test]
    fn test_no_overlap() {
        let result = calculate_unique_total_time(&mut [
            TimeEvent::new(true, 10),
            TimeEvent::new(false, 20),
            TimeEvent::new(true, 20),
            TimeEvent::new(false, 30),
            TimeEvent::new(true, 40),
            TimeEvent::new(false, 50),
        ]);

        assert_eq!(result, 30);
    }

    #[test]
    fn test_overlap() {
        let result = calculate_unique_total_time(&mut [
            TimeEvent::new(true, 10),
            TimeEvent::new(false, 20),
            TimeEvent::new(true, 15),
            TimeEvent::new(false, 30),
            TimeEvent::new(true, 25),
            TimeEvent::new(false, 40),
        ]);

        assert_eq!(result, 30);
    }

    #[test]
    fn test_overlap_unsorted() {
        let result = calculate_unique_total_time(&mut [
            TimeEvent::new(false, 30),
            TimeEvent::new(true, 15),
            TimeEvent::new(false, 20),
            TimeEvent::new(true, 10),
            TimeEvent::new(true, 25),
            TimeEvent::new(false, 40),
        ]);

        assert_eq!(result, 30);
    }

    #[test]
    fn test_complex() {
        let result = calculate_unique_total_time(&mut [
            TimeEvent::new(true, 1),
            TimeEvent::new(false, 43),
            TimeEvent::new(true, 20),
            TimeEvent::new(false, 34),
            TimeEvent::new(true, 50),
            TimeEvent::new(false, 70),
            TimeEvent::new(true, 100),
            TimeEvent::new(false, 120),
            TimeEvent::new(true, 110),
            TimeEvent::new(false, 150),
            TimeEvent::new(true, 80),
            TimeEvent::new(false, 85),
            TimeEvent::new(true, 56),
            TimeEvent::new(false, 77),
        ]);

        assert_eq!(result, 124);
    }
}
