use std::fmt;
use std::str;

/// Type of event that may occur on a work item.
#[derive(Debug, Copy, Clone)]
pub enum EventType {
    /// The work item has been started.
    Started,
    /// The work item has been paused.
    Paused,
    /// Work on the work item has continued.
    Continued,
    /// Work on the work item has finished.
    Finished,
}

impl str::FromStr for EventType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STARTED" => Ok(EventType::Started),
            "PAUSED" => Ok(EventType::Paused),
            "CONTINUED" => Ok(EventType::Continued),
            "FINISHED" => Ok(EventType::Finished),
            _ => Err(()),
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Started => write!(f, "STARTED"),
            EventType::Paused => write!(f, "PAUSED"),
            EventType::Continued => write!(f, "CONTINUED"),
            EventType::Finished => write!(f, "FINISHED"),
        }
    }
}
