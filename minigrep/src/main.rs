use std::env;
use std::process;
use libminigrep::ProgramArguments;

fn main() {
    //v.iter().map(AsRef::as_ref).collect();
    // let args : Vec<String> = env::args().collect();
    // let args: Vec<&str> = args.iter().map(|x| &**x).collect();

    let parsed_result = ProgramArguments::new(env::args());

    let _result = parsed_result.unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    if let Err(e) = libminigrep::run(_result) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
