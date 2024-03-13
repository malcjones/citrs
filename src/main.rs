use citrs::sh;
use colored::Colorize;

fn main() {
    let mut my_shell = sh::Shell {
        prompt: |s| {
            match s.state {
                sh::State::Error(_) => "! ".red().to_string(),
                _ => "> ".to_string()
            }
        },
        state: sh::State::Ok,
        mode: 0,
        modes: vec![
        ],
        builtin: vec![
            sh::cmd::Command {
                name: "debug",
                description: "prints debug information",
                usage: "debug [args]",
                action: |shell, args| {
                    println!("state: {:?}", shell.state);
                    println!("mode: {:?}", shell.get_mode());
                    if !args.is_empty() {
                        println!("args: {:?}", args)
                    }
                    Ok(())
                }
            },
            sh::cmd::Command {
                name: "exit",
                description: "exit the shell",
                usage: "exit",
                action: |shell, _| {
                    shell.ok();
                    std::process::exit(0)
                }
            },
            sh::cmd::Command {
                name: "help",
                description: "prints help information",
                usage: "help [command]",
                action: |shell, args| {
                    if args.is_empty() {
                        println!("commands:");
                        for command in &shell.commands() {
                            println!("  {:<10} {}", command.name, command.description)
                        }
                    } else {
                        let command = shell.find_command(&args[0])?;
                        println!("{}: {}", command.name, command.description);
                        println!("usage: {}", command.usage)
                    }
                    Ok(())
                }
            },
            sh::cmd::Command {
                name: "mode",
                description: "change the shell mode",
                usage: "mode <name>",
                action: |shell, args| {
                    if args.is_empty() {
                        println!("mode: {}", shell.get_mode().map(|m| m.name.clone()).unwrap_or("none".to_string()));
                        if !shell.modes.is_empty() {
                            println!("modes:");
                            for mode in &shell.modes {
                                println!("  {}", mode.name)
                            }
                        }
                        return Ok(())
                    }
                    shell.mode = shell.find_mode_index(&args[0]).ok_or("mode not found")?;
                    Ok(())
                }
            }
        ]
    };
    my_shell.run()
}

