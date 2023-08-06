use crate::command;
use owo_colors::OwoColorize;

pub fn run() {
    todo_lib::utils::init();

    let args = command::Command::new();

    match args.command().clone().as_str() {
        "a" => todo_lib::todo::Todo::add(args.argument()),
        "add" => todo_lib::todo::Todo::add(args.argument()),
        "l" => todo_lib::todo::Todo::list(),
        "ls" => todo_lib::todo::Todo::list(),
        "list" => todo_lib::todo::Todo::list(),
        "r" => todo_lib::todo::Todo::remove(args.argument()),
        "rm" => todo_lib::todo::Todo::remove(args.argument()),
        "remove" => todo_lib::todo::Todo::remove(args.argument()),
        "d" => todo_lib::todo::Todo::done(args.argument()),
        "done" => todo_lib::todo::Todo::done(args.argument()),

        "q" => std::process::exit(0),
        "quit" => std::process::exit(0),

        _ => {
            help();
        }
    }
}

fn help() {
    println!("{}", "            No command found - Showing help".black());

    let help = format!(
        "
            {} {}
            {}
            -----

            Help:

            Command   | Arguments | Description
            {}           text        Add a new todo
            {}                       List all todos
            {}           id          Mark a todo as done
            {}           id          Delete a todo
            {}                       Quit
        ",
        "Welcome to",
        "Rodos".cyan(),
        "Simple todo app written in Rust".black(),
        "a".cyan(),
        "l".blue(),
        "d".green(),
        "r".red(),
        "q".black()
    );

    println!("{help}");
}
