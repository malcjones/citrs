use colored::Colorize;
use rustyline::error::ReadlineError;

pub mod cmd;
pub mod mode;

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Ok,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Shell {
    pub state: State,
    pub mode: usize,
    pub modes: Vec<mode::Mode>,
    pub prompt: fn(&Shell) -> String,
    pub builtin: Vec<cmd::Command>,
}

pub enum ShellError {
    CommandError,
}

impl Shell {
    /// Create a new shell
    pub fn new() -> Shell {
        Shell {
            state: State::Ok,
            mode: 0,
            modes: Vec::new(),
            prompt: |s| match s.state {
                State::Error(_) => "! ".red().to_string(),
                _ => "> ".to_string(),
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
    pub fn get_mode(&self) -> Option<&mode::Mode> {
        self.modes.get(self.mode)
    }

    // Find the index of a mode by name
    pub fn find_mode_index(&self, name: &str) -> Option<usize> {
        self.modes.iter().position(|m| m.name == name)
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
            .find(|c| c.name == name)
            .map(|c| c.clone())
            .ok_or(format!("command not found: {}", name))
    }

    pub fn commands(&self) -> Vec<cmd::Command> {
        let mut commands = self.builtin.clone();
        if let Some(mode) = self.get_mode() {
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

    /// Handle a line of input
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
}

/// Parse a line of input into a command name and arguments
/// 
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
    let name = cmd_args
        .next()
        .ok_or("no command provided")?
        .to_string();
    if name.is_empty() {
        return Err("no command provided".to_string());
    }
    let args: Vec<String> = cmd_args.next().map_or(Vec::new(), |s| s.split(" ").map(|s| s.to_string()).collect());
    Ok((
        name,
        args
    ))
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
