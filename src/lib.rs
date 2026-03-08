pub mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub mod cli;
pub use cli::commands;
pub use cli::{
    BootstrapCommand, // BootstrapCommand
    BootstrapDirOpt,  // BootstrapDirOpt
    BootstrapFileOpt, // BootstrapFileOpt
    BootstrapOpt,     // BootstrapOpt

    BootstrapSharedOpt, // BootstrapSharedOpt
    ClientCommand,      // ClientCommand
    ClientDirOpt,       // ClientDirOpt
    ClientFileOpt,      // ClientFileOpt
    ClientOpt,          // ClientOpt

    ClientSharedOpt, // ClientSharedOpt
    ContextCommand,  // ContextCommand
    ContextDirOpt,   // ContextDirOpt
    ContextFileOpt,  // ContextFileOpt
    ContextOpt,      // ContextOpt

    ContextSharedOpt, // ContextSharedOpt
    DeleteCommand,    // DeleteCommand
    DeleteDirOpt,     // DeleteDirOpt
    DeleteFileOpt,    // DeleteFileOpt
    DeleteOpt,        // DeleteOpt

    DeleteSharedOpt, // DeleteSharedOpt
    DoctorCommand,   // DoctorCommand
    DoctorDirOpt,    // DoctorDirOpt
    DoctorFileOpt,   // DoctorFileOpt
    DoctorOpt,       // DoctorOpt

    DoctorSharedOpt, // DoctorSharedOpt
    EditCommand,     // EditCommand
    EditDirOpt,      // EditDirOpt
    EditFileOpt,     // EditFileOpt
    EditOpt,         // EditOpt

    EditSharedOpt,  // EditSharedOpt
    EnqueueCommand, // EnqueueCommand
    EnqueueDirOpt,  // EnqueueDirOpt
    EnqueueFileOpt, // EnqueueFileOpt
    EnqueueOpt,     // EnqueueOpt

    EnqueueSharedOpt, // EnqueueSharedOpt
    EnvCommand,       // EnvCommand
    EnvDirOpt,        // EnvDirOpt
    EnvFileOpt,       // EnvFileOpt
    EnvOpt,           // EnvOpt
    EnvSharedOpt,     // EnvSharedOpt

    ExportCommand,   // ExportCommand
    ExportDirOpt,    // ExportDirOpt
    ExportFileOpt,   // ExportFileOpt
    ExportOpt,       // ExportOpt
    ExportSharedOpt, // ExportSharedOpt

    FindCommand,   // FindCommand
    FindDirOpt,    // FindDirOpt
    FindFileOpt,   // FindFileOpt
    FindOpt,       // FindOpt
    FindSharedOpt, // FindSharedOpt
    GotoCommand,   // GotoCommand

    GotoDirOpt,    // GotoDirOpt
    GotoFileOpt,   // GotoFileOpt
    GotoOpt,       // GotoOpt
    GotoSharedOpt, // GotoSharedOpt
    ImportCommand, // ImportCommand

    ImportDirOpt,    // ImportDirOpt
    ImportFileOpt,   // ImportFileOpt
    ImportOpt,       // ImportOpt
    ImportSharedOpt, // ImportSharedOpt
    InitCommand,     // InitCommand

    InitDirOpt,    // InitDirOpt
    InitFileOpt,   // InitFileOpt
    InitOpt,       // InitOpt
    InitSharedOpt, // InitSharedOpt
    ListCommand,   // ListCommand
    ListDirOpt,    // ListDirOpt

    ListFileOpt,   // ListFileOpt
    ListOpt,       // ListOpt
    ListSharedOpt, // ListSharedOpt
    LoadCommand,   // LoadCommand
    LoadDirOpt,    // LoadDirOpt
    LoadFileOpt,   // LoadFileOpt

    LoadOpt,       // LoadOpt
    LoadSharedOpt, // LoadSharedOpt
    ParseCommand,  // ParseCommand
    ParseDirOpt,   // ParseDirOpt
    ParseFileOpt,  // ParseFileOpt
    ParseOpt,      // ParseOpt

    ParseSharedOpt, // ParseSharedOpt
    PathCommand,    // PathCommand
    PathDirOpt,     // PathDirOpt
    PathFileOpt,    // PathFileOpt
    PathOpt,        // PathOpt

    PathSharedOpt, // PathSharedOpt
    ReadCommand,   // ReadCommand
    ReadDirOpt,    // ReadDirOpt
    ReadFileOpt,   // ReadFileOpt
    ReadOpt,       // ReadOpt

    ReadSharedOpt,  // ReadSharedOpt
    RefreshCommand, // RefreshCommand
    RefreshDirOpt,  // RefreshDirOpt
    RefreshFileOpt, // RefreshFileOpt
    RefreshOpt,     // RefreshOpt

    RefreshSharedOpt, // RefreshSharedOpt
    SaveCommand,      // SaveCommand
    SaveDirOpt,       // SaveDirOpt
    SaveFileOpt,      // SaveFileOpt
    SaveOpt,          // SaveOpt

    SaveSharedOpt, // SaveSharedOpt
    ServerCommand, // ServerCommand
    ServerDirOpt,  // ServerDirOpt
    ServerFileOpt, // ServerFileOpt
    ServerOpt,     // ServerOpt

    ServerSharedOpt, // ServerSharedOpt
    ShellCommand,    // ShellCommand
    ShellDirOpt,     // ShellDirOpt
    ShellFileOpt,    // ShellFileOpt
    ShellOpt,        // ShellOpt

    ShellSharedOpt, // ShellSharedOpt
    ShowCommand,    // ShowCommand
    ShowDirOpt,     // ShowDirOpt
    ShowFileOpt,    // ShowFileOpt
    ShowOpt,        // ShowOpt

    ShowSharedOpt, // ShowSharedOpt
    StashCommand,  // StashCommand
    StashDirOpt,   // StashDirOpt
    StashFileOpt,  // StashFileOpt
    StashOpt,      // StashOpt

    StashSharedOpt, // StashSharedOpt
    SwitchCommand,  // SwitchCommand
    SwitchDirOpt,   // SwitchDirOpt
    SwitchFileOpt,  // SwitchFileOpt
    SwitchOpt,      // SwitchOpt

    SwitchSharedOpt, // SwitchSharedOpt
    TaskCommand,     // TaskCommand
    TaskDirOpt,      // TaskDirOpt
    TaskFileOpt,     // TaskFileOpt
    TaskOpt,         // TaskOpt

    TaskSharedOpt, // TaskSharedOpt
    TodayCommand,  // TodayCommand
    TodayDirOpt,   // TodayDirOpt
    TodayFileOpt,  // TodayFileOpt
    TodayOpt,      // TodayOpt

    TodaySharedOpt, // TodaySharedOpt
    ToolCommand,    // ToolCommand
    ToolDirOpt,     // ToolDirOpt
    ToolFileOpt,    // ToolFileOpt
    ToolOpt,        // ToolOpt

    ToolSharedOpt, // ToolSharedOpt
    UpdateCommand, // UpdateCommand
    UpdateDirOpt,  // UpdateDirOpt
    UpdateFileOpt, // UpdateFileOpt
    UpdateOpt,     // UpdateOpt

    UpdateSharedOpt, // UpdateSharedOpt
    WebCommand,      // WebCommand
    WebDirOpt,       // WebDirOpt
    WebFileOpt,      // WebFileOpt
    WebOpt,          // WebOpt
    WebSharedOpt,    // WebSharedOpt

    WriteCommand,   // WriteCommand
    WriteDirOpt,    // WriteDirOpt
    WriteFileOpt,   // WriteFileOpt
    WriteOpt,       // WriteOpt
    WriteSharedOpt, // WriteSharedOpt
};
