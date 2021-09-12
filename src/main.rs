mod models;
mod parser;
mod pretty_printer;

use crate::models::Function;
use crate::parser::get_functions;
use crate::pretty_printer::print_functions;
use colored::*;
use structopt::StructOpt;

/// Run or list the contents of a script
#[derive(StructOpt)]
struct Cli {
    /// The path to the script to describe or run.
    #[structopt(parse(from_os_str))]
    script: std::path::PathBuf,
    /// The name of the function to run. This will not run the function, it will just validate that it exists.
    function: Option<String>,
}

fn main() {
    let args = Cli::from_args();
    match get_functions(&args.script) {
        Ok(functions) => match &args.function {
            Some(function_to_run) => {
                match functions.iter().find(|&n| &n.name == function_to_run) {
                    Some(_) => {
                        // Found a valid function. We're going to return a non-0 exit code
                        // so the script knows that it can go ahead and run the function.
                        std::process::exit(78);
                    }
                    None => {
                        let script = &args.script.into_os_string().into_string().unwrap();
                        println!("{}", "Function does not exist!\n".red());
                        print_functions(functions, script);
                    }
                }
            }
            None => {
                let script = &args.script.into_os_string().into_string().unwrap();
                print_functions(functions, script);
            }
        },
        Err(_) => {
            let script = &args.script.into_os_string().into_string().unwrap();
            println!(
                "{} {}",
                "Unable to get functions from".red(),
                script.green()
            );
        }
    }
}
