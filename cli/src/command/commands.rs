use crate::command::command::Command;
use crate::command::continue_cmd::ContinueCommand;
use crate::command::export::ExportCommand;
use crate::command::list::ListCommand;
use crate::command::log::LogCommand;
use crate::command::pause::PauseCommand;
use crate::command::start::StartCommand;
use crate::command::stop::StopCommand;

/// All available commands.
pub(crate) const COMMANDS: [&dyn Command; 7] = [
    &ListCommand {},
    &LogCommand {},
    &StartCommand {},
    &StopCommand {},
    &PauseCommand {},
    &ContinueCommand {},
    &ExportCommand {},
];
