use git_bandolier::{Exit, cli::commands::BranchesOpt, dispatch::ParserDispatcher};

fn main() -> Exit {
    BranchesOpt::main()
}
