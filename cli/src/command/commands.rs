use crate::command::command::Command;
use crate::command::continue_cmd::ContinueCommand;
use crate::command::export::ExportCommand;
use crate::command::finish::FinishCommand;
use crate::command::list::ListCommand;
use crate::command::log::LogCommand;
use crate::command::pause::PauseCommand;
use crate::command::start::StartCommand;

/// All available commands.
pub(crate) const COMMANDS: [&dyn Command; 7] = [
    &ListCommand {},
    &LogCommand {},
    &StartCommand {},
    &FinishCommand {},
    &PauseCommand {},
    &ContinueCommand {},
    &ExportCommand {},
];
