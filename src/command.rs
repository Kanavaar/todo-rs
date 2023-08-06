pub struct Command {
    command: String,
    argument: String,
}

impl Command {
    pub fn new() -> Command {
        let args = std::env::args().collect::<Vec<String>>();

        let default = "".to_string();
        let command = (match args.get(1) {
            Some(command) => command,
            None => &default,
        })
        .to_string();
        let argument = (match args.get(2) {
            Some(arg) => arg,
            None => &default,
        })
        .to_string();
        Command { command, argument }
    }

    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn argument(&self) -> String {
        self.argument.clone()
    }
}
