use clap::{Parser, Subcommand};
use workbench::cli::commands::{
    BootstrapOpt,
    // ClientOpt,
    // ContextOpt,
    // DeleteOpt,
    // DoctorOpt,
    // EditOpt,
    // EnvOpt,
    // ExportOpt,
    // FindOpt,
    // GotoOpt,
    // ImportOpt,
    // InitOpt,
    // ListOpt,
    // LoadOpt,
    // ParseOpt,
    // PathOpt,
    // ReadOpt,
    // RefreshOpt,
    // SaveOpt,
    // ServerOpt,
    // ShellOpt,
    // ShowOpt,
    // StashOpt,
    // SwitchOpt,
    // TodayOpt,
    // ToolOpt,
    // UpdateOpt,
    // WebOpt,
    // WriteOpt,
};
use workbench::dispatch::{
    ArgsDispatcher, ParserDispatcher, SubcommandDispatcher,
};
use workbench::{Error, Exit, Result};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "workbench command-line"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}
impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Bootstrap(BootstrapOpt),
    // Env(EnvOpt),
    // Context(ContextOpt),
    // Switch(SwitchOpt),
    // Path(PathOpt),
    // Goto(GotoOpt),
    // List(ListOpt),
    // Init(InitOpt),
    // Doctor(DoctorOpt),
    // Find(FindOpt),
    // Show(ShowOpt),
    // Today(TodayOpt),
    // Update(UpdateOpt),
    // Delete(DeleteOpt),
    // Edit(EditOpt),
    // Server(ServerOpt),
    // Client(ClientOpt),
    // Refresh(RefreshOpt),
    // Shell(ShellOpt),
    // Tool(ToolOpt),
    // Parse(ParseOpt),
    // Export(ExportOpt),
    // Import(ImportOpt),
    // Web(WebOpt),
    // Stash(StashOpt),
    // Save(SaveOpt),
    // Load(LoadOpt),
    // Write(WriteOpt),
    // Read(ReadOpt),
}
impl SubcommandDispatcher<Error> for Command {
    fn dispatch(&self) -> Result<()> {
        match self {
            Command::Bootstrap(op) => op.dispatch()?,
            // Command::Env(op) => op.dispatch()?,
            // Command::Context(op) => op.dispatch()?,
            // Command::Switch(op) => op.dispatch()?,
            // Command::Path(op) => op.dispatch()?,
            // Command::Sh(op) => op.dispatch()?,
            // Command::Goto(op) => op.dispatch()?,
            // Command::List(op) => op.dispatch()?,
            // Command::Init(op) => op.dispatch()?,
            // Command::Doctor(op) => op.dispatch()?,
            // Command::Find(op) => op.dispatch()?,
            // Command::Show(op) => op.dispatch()?,
            // Command::Today(op) => op.dispatch()?,
            // Command::Update(op) => op.dispatch()?,
            // Command::Delete(op) => op.dispatch()?,
            // Command::Edit(op) => op.dispatch()?,
            // Command::Server(op) => op.dispatch()?,
            // Command::Client(op) => op.dispatch()?,
            // Command::Refresh(op) => op.dispatch()?,
            // Command::Shell(op) => op.dispatch()?,
            // Command::Tool(op) => op.dispatch()?,
            // Command::Parse(op) => op.dispatch()?,
            // Command::Export(op) => op.dispatch()?,
            // Command::Import(op) => op.dispatch()?,
            // Command::Web(op) => op.dispatch()?,
            // Command::Stash(op) => op.dispatch()?,
            // Command::Save(op) => op.dispatch()?,
            // Command::Load(op) => op.dispatch()?,
            // Command::Write(op) => op.dispatch()?,
            // Command::Read(op) => op.dispatch()?,
        }
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
