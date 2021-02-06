/// Command usable using the worklog CLI.
pub(crate) trait Command {
    /// Build the command group for the command line parser.
    fn build(&self) -> cmd_args::Group;

    /// Get all available command aliases.
    fn aliases(&self) -> Option<Vec<&str>>;

    /// Command name.
    fn name(&self) -> &str;
}
