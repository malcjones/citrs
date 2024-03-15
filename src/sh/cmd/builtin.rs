use super::{
    builder::{Arg, CommandBuilder},
    Command,
};

/// Get the default builtin commands
pub fn default() -> Vec<Command> {
    vec![debug(), help(), mode()]
}

fn debug() -> Command {
    CommandBuilder::new("debug", "print debug information")
        .action(|shell, args| {
            println!("state: {:?}", shell.state);
            if !args.is_empty() {
                println!("args: {:?}", args);
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
                if let Some(mode) = shell.mode() {
                    for command in mode.commands.iter() {
                        println!("{} - {} ({})", command.name, command.description, mode.name)
                    }
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

fn mode() -> Command {
    CommandBuilder::new("mode", "change or print the current mode")
        .action(|shell, args| {
            if args.is_empty() {
                let current_name = shell.mode().map_or("none", |m| &m.name);
                println!("mode: {}", current_name);
                if shell.modes.len() > 1 {
                    for mode in shell.modes.iter() {
                        if mode.name != current_name {
                            println!("   {}", mode.name)
                        }
                    }
                }
            } else if let Some(idx) = shell.modes.iter().position(|m| m.name == args[0]) {
                shell.current_mode = Some(idx)
            } else if args[0] == "none" {
                shell.current_mode = None
            } else {
                return Err(format!("no mode '{}'", args[0]));
            }
            Ok(())
        })
        .arg(Arg::Optional("name | none".to_string()))
        .command
}
