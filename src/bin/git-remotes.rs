use git_bandolier::{Exit, cli::commands::RemotesOpt, dispatch::ParserDispatcher};

fn main() -> Exit {
    RemotesOpt::main()
}
