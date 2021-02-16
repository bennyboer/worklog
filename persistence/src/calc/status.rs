use std::fmt;
use std::str;

/// Status of a log calc.
#[derive(Debug, Copy, Clone)]
pub enum Status {
    Done,
    InProgress,
    Paused,
}

impl str::FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DONE" => Ok(Status::Done),
            "IN_PROGRESS" => Ok(Status::InProgress),
            "PAUSED" => Ok(Status::Paused),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Done => write!(f, "DONE"),
            Status::InProgress => write!(f, "IN_PROGRESS"),
            Status::Paused => write!(f, "PAUSED"),
        }
    }
}
