use clap::{arg, command, Command};

fn main() {
    // Create the CLI application
    let matches = command!()
        // Show usage, options and subcommands
        .arg_required_else_help(true)
        // Add a new subcommand
        .subcommand(
            // Create a new subcommand with the name "greeting"
            Command::new("greeting")
                // Add a subcommand's description
                .about("Simple hello world")
                // Add first argument
                .arg(arg!(--greet <VALUE>))
                // Add second subcommand
                .arg(arg!(--name <VALUE>)),
        )
        // get the matches
        .get_matches();

    match matches.subcommand() {
        // If the user used the subcommand "greeting"
        Some(("greeting", sub_matches)) => {
            println!(
                "Greeting: {}, {}",
                // Use the value of the --greet argument
                sub_matches.get_one::<String>("greet").expect("required"),
                // Use the value of the --name argument
                sub_matches.get_one::<String>("name").expect("required")
            )
        }
        // Any other subcommand will be handled by the default handler
        _ => unreachable!("Unsupported subcommands."),
    }
}
