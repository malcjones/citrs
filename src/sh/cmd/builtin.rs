use super::{
    builder::{Arg, CommandBuilder},
    Command,
};

/// Get the default builtin commands
pub fn default() -> Vec<Command> {
    vec![debug(), help()]
}

fn debug() -> Command {
    CommandBuilder::new("debug", "print debug information")
        .action(|shell, args| {
            println!("{:?}", shell.state);
            if let Some(mode) = shell.mode() {
                println!("Mode: {}", mode.name);
            } else {
                println!("No mode");
            }
            if !args.is_empty() {
                println!("Args: {:?}", args);
            }
            Ok(())
        })
        .arg(Arg::Optional("args".to_string()))
        .command
}

fn help() -> Command {
    CommandBuilder::new("help", "print help information")
        .action(|shell, args| {
            if args.is_empty() {
                for command in shell.builtin.iter() {
                    println!("{} - {}", command.name, command.description);
                }
            } else {
                let command = shell.find_command(&args[0])?;
                println!("{} - {}", command.name, command.description);
                println!("Usage: {}", command.usage);
            }
            Ok(())
        })
        .arg(Arg::Optional("command".to_string()))
        .command
}
