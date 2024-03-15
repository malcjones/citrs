use citrs::sh::cmd::builder::{Arg, CommandBuilder};
use citrs::sh::cmd::Command;
use citrs::sh::mode::Mode;

pub fn populate(shell: &mut citrs::sh::Shell) {
    shell.builtin = commands::all();
    shell.modes = modes::all();
    shell.set_mode(Some(0));
}

pub mod modes {
    use super::{Command, CommandBuilder, Mode};

    pub fn all() -> Vec<Mode> {
        vec![bookmark::mode()]
    }

    pub mod bookmark {
        use super::{Command, CommandBuilder, Mode};

        pub fn mode() -> Mode {
            Mode {
                name: "bookmark".to_string(),
                description: "Bookmark mode".to_string(),
                commands: vec![add(), list()],
            }
        }

        fn add() -> Command {
            CommandBuilder::todo("add").build()
        }

        fn list() -> Command {
            CommandBuilder::todo("list").build()
        }
    }
}

pub mod commands {
    use super::{Arg, Command, CommandBuilder};

    pub fn all() -> Vec<Command> {
        vec![debug(), help(), mode()]
    }

    fn debug() -> Command {
        CommandBuilder::new("debug", "print debug information")
            .action(|shell, args| {
                if args.is_empty() {
                    println!("state: {:?}", shell.state);
                    println!("builtin: {}", shell.builtin.len());
                    if let Some(mode) = shell.mode() {
                        println!("mode: {}", mode.name);
                        println!("  '{}'", mode.description);
                        println!("  commands: {}", mode.commands.len());
                    } else {
                        println!("no mode");
                    }
                }

                Ok(())
            })
            .arg(Arg::Optional("cmd".to_string()))
            .aliases(vec!["dbg", "!"])
            .build()
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
                    if command.aliases.len() > 1 {
                        println!("aliases: {}", command.aliases[1..].join(", "));
                    }
                    println!("Usage: {}", command.usage);
                }
                Ok(())
            })
            .arg(Arg::Optional("command".to_string()))
            .alias("?")
            .build()
    }

    fn mode() -> Command {
        CommandBuilder::new("mode", "change or print the current mode")
            .action(|shell, args| {
                if args.is_empty() {
                    let current_name = shell.mode().map_or("none", |m| &m.name);
                    println!("mode: {}", current_name);
                    if shell.modes.len() > 1 || (shell.mode().is_none() && !shell.modes.is_empty())
                    {
                        println!("available:");
                        for mode in shell.modes.iter() {
                            if mode.name != current_name {
                                println!("  - {}", mode.name)
                            }
                        }
                    } else {
                        println!("no other modes");
                    }
                } else if let Some(idx) = shell.modes.iter().position(|m| m.name == args[0]) {
                    shell.set_mode(Some(idx));
                } else if args[0] == "none" {
                    shell.set_mode(None);
                } else {
                    return Err(format!("no mode '{}'", args[0]));
                }
                Ok(())
            })
            .arg(Arg::Optional("name | none".to_string()))
            .build()
    }
}
