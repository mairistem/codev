//! `ledger`: reads the command line and runs the command it names.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match args.as_slice() {
        [command, path] if command == "import" => ledger::cli::import::run(path),
        _ => {
            eprintln!("usage: ledger import <file>");
            2
        }
    };
    std::process::exit(code);
}
