use git_bandolier::{Exit, cli::commands::CommitDatedOpt, dispatch::ParserDispatcher};

fn main() -> Exit {
    CommitDatedOpt::main()
}
