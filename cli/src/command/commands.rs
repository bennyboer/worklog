use crate::command::clear::ClearCommand;
use crate::command::command::Command;
use crate::command::continue_cmd::ContinueCommand;
use crate::command::delete::DeleteCommand;
use crate::command::edit::EditCommand;
use crate::command::export::ExportCommand;
use crate::command::finish::FinishCommand;
use crate::command::list::ListCommand;
use crate::command::log::LogCommand;
use crate::command::pause::PauseCommand;
use crate::command::show::ShowCommand;
use crate::command::start::StartCommand;

/// All available commands.
pub(crate) const COMMANDS: [&dyn Command; 11] = [
    &ListCommand {},
    &LogCommand {},
    &StartCommand {},
    &FinishCommand {},
    &PauseCommand {},
    &ContinueCommand {},
    &ExportCommand {},
    &DeleteCommand {},
    &EditCommand {},
    &ClearCommand {},
    &ShowCommand {},
];
