use crate::{
    executables::Executables,
    script::{Function, Script},
};
use colored::Colorize;

pub fn print_root_header() {
    println!("{}", "lk: ./".on_blue());
}

pub fn print_script_header(script: &Script) {
    let script_path = script
        .path
        .to_owned()
        .into_os_string()
        .into_string()
        .unwrap();
    println!("{}{}", "lk: ".on_blue(), script_path.on_blue());
}

pub fn print_complete_header(script: &Script, function: &Function, params: &[String]) {
    println!(
        "{}{}{}{}{}{}{}",
        "lk: ".on_blue(),
        script.path.as_os_str().to_string_lossy().on_blue(),
        " -> ".on_blue(),
        function.name.on_blue(),
        " (".on_blue(),
        params.join(" ").on_blue(),
        ")".on_blue()
    );
}

pub fn print_no_functions_in_script_help() {
    println!("Could not find any functions! Why not add some. They look like this:");
    let example_function = r#"# Some great comment
# More insightful and fascinating insights into bash scripting
blow_mind() {
    echo "OMG so cool"
} "#;
    println!("{}", example_function.italic());
}

pub fn print_bad_script_name(script: &str, executables: Executables) {
    println!(
        "{} {}!\n",
        "Didn't find a script with name".red(),
        script.blue()
    );
    executables.pretty_print();
}

pub fn print_bad_function_name(script: &Script, function: &str) {
    println!(
        "{} {}!\n",
        "Didn't find a function with name".red(),
        function.blue()
    );
    script.pretty_print();
}