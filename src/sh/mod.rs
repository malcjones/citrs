use colored::Colorize;
use rustyline::error::ReadlineError;
use std::collections::HashMap;

pub mod cmd;
pub mod flag;
pub mod mode;

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Ok,
    Error(String),
}

#[derive(Debug)]
pub struct Shell {
    pub state: State,
    flags: HashMap<String, flag::Flag>,
    current_mode: Option<usize>,
    pub modes: Vec<mode::Mode>,
    prompt: PromptGen,
    pub builtin: Vec<cmd::Command>,
}

pub enum ShellError {
    CommandError,
}

pub type PromptGen = fn(&Shell) -> String;

impl Shell {
    /// Create a new shell
    pub fn new() -> Shell {
        Shell {
            state: State::Ok,
            current_mode: None,
            modes: Vec::new(),
            flags: HashMap::new(),
            prompt: |s| {
                match s.state {
                    State::Error(_) => "! ".red(),
                    _ => "> ".green(),
                }
                .to_string()
            },
            builtin: Vec::new(),
        }
    }

    /// Set the shell state to an error
    ///
    /// # Example
    /// ```
    /// use citrs::sh::{Shell, State};
    ///
    /// let mut shell = Shell::new();
    /// shell.err("error message".to_string());
    /// assert_eq!(shell.state, State::Error("error message".to_string()));
    /// ```
    pub fn err(&mut self, message: String) {
        self.state = State::Error(message)
    }

    /// Reset the shell state
    ///
    /// # Example
    /// ```
    /// use citrs::sh::{Shell, State};
    ///
    /// let mut shell = Shell::new();
    /// shell.err("error message".to_string());
    /// shell.ok();
    /// assert_eq!(shell.state, State::Ok);
    /// ```
    pub fn ok(&mut self) {
        self.state = State::Ok
    }

    /// Get the current mode
    pub fn mode(&self) -> Option<&mode::Mode> {
        self.modes.get(self.current_mode?)
    }

    /// Find a command by name
    ///
    /// # Example
    /// ```
    /// use citrs::sh::{Shell, cmd};
    ///
    /// let mut shell = Shell::new();
    /// let command = cmd::Command {
    ///    name: "test",
    ///    description: "Test command",
    ///    usage: "test <arg>",
    ///    action: |_, _| Ok(())
    /// };
    ///
    /// shell.builtin.push(command.clone());
    /// let result = shell.find_command("test").unwrap();
    /// assert_eq!(result, &command);
    /// ```
    pub fn find_command(&self, name: &str) -> Result<cmd::Command, String> {
        self.commands()
            .iter()
            .find(|c| c.aliases.contains(&name.to_string()))
            .cloned()
            .ok_or(format!("command not found: {}", name))
    }

    pub fn commands(&self) -> Vec<cmd::Command> {
        let mut commands = self.builtin.clone();
        if let Some(mode) = self.mode() {
            commands.extend(mode.commands.clone());
        }
        commands
    }

    /// Take a line of input from the user
    pub fn take_line(&self, editor: &mut rustyline::DefaultEditor) -> Result<String, String> {
        let line = editor
            .readline((self.prompt)(self).as_str())
            .map_err(|e| match e {
                ReadlineError::Interrupted => {
                    eprintln!("! CTRL-C");
                    std::process::exit(1);
                }
                ReadlineError::Eof => {
                    eprintln!("! EOF");
                    std::process::exit(1);
                }
                _ => "couldn't take line".to_string(),
            })?;
        editor
            .add_history_entry(&line)
            .or(Err("couldn't add line to history"))?;
        Ok(line)
    }

    /// Handle a line of input, running it if it resolves to a functioning command
    pub fn handle_line(&mut self, line: String) -> Result<(), String> {
        let (command_name, args) = parse_line(line)?;
        let command = self.find_command(&command_name)?;
        command.clone().run(self, args)
    }

    /// Run the shell
    pub fn run(&mut self) {
        let mut editor = rustyline::DefaultEditor::new().unwrap();
        loop {
            let line = match self.take_line(&mut editor) {
                Ok(line) => line,
                Err(e) => {
                    println!("err: {}", e);
                    self.err(e);
                    continue;
                }
            };
            if let Err(e) = self.handle_line(line) {
                println!("err: {}", e);
                self.err(e);
                continue;
            }
            self.ok()
        }
    }

    pub fn set_mode(&mut self, idx: Option<usize>) {
        match idx {
            Some(idx) => {
                if idx < self.modes.len() {
                    self.current_mode = Some(idx)
                }
            }
            None => self.current_mode = None,
        }
    }

    pub fn set_flag(&mut self, name: &str, value: flag::Flag) {
        self.flags.insert(name.to_string(), value);
    }

    pub fn get_flag(&self, name: &str) -> Option<&flag::Flag> {
        self.flags.get(name)
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell::new()
    }
}

/// Parse a line of input into a command name and arguments
/// TODO: Quotes
/// # Example
/// ```
/// use citrs::sh::parse_line;
///
/// let result = parse_line("command arg1 arg2".to_string());
/// assert!(result.is_ok());
/// let (command_name, args) = result.unwrap();
/// assert_eq!(command_name, "command");
/// assert_eq!(args, vec!["arg1", "arg2"]);
/// ```
pub fn parse_line(line: String) -> Result<(String, Vec<String>), String> {
    let mut cmd_args = line.splitn(2, ' ');
    let name = cmd_args.next().ok_or("no command provided")?.to_string();
    if name.is_empty() {
        return Err("no command provided".to_string());
    }
    let args: Vec<String> = cmd_args.next().map_or(Vec::new(), |s| {
        let mut in_quotes = false;
        let mut in_arg = false;
        let mut arg = String::new();
        let mut args = Vec::new();

        for c in s.chars() {
            match c {
                ' ' if !in_quotes => {
                    if in_arg {
                        args.push(arg.clone());
                        arg.clear();
                        in_arg = false;
                    }
                }
                '"' => {
                    in_quotes = !in_quotes;
                    if in_arg {
                        args.push(arg.clone());
                        arg.clear();
                        in_arg = false;
                    }
                }
                _ => {
                    in_arg = true;
                    arg.push(c);
                }
            }
        }

        if in_arg {
            args.push(arg);
        }

        args
    });
    Ok((name, args))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_valid() {
        let line = String::from("command arg1 arg2");
        let result = parse_line(line);
        assert!(result.is_ok());
        let (command_name, args) = result.unwrap();
        assert_eq!(command_name, "command");
        assert_eq!(args, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_parse_line_no_args() {
        let line = String::from("command");
        let result = parse_line(line);
        assert!(result.is_ok());
        let (command_name, args) = result.unwrap();
        assert_eq!(command_name, "command");
        assert_eq!(args, Vec::<String>::new());
    }

    #[test]
    fn test_parse_line_empty() {
        let line = String::from("");
        let result = parse_line(line);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "no command provided");
    }
}
