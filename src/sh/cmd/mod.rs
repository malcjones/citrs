use super::Shell;

pub mod builtin;
pub mod builder;

#[derive(Clone, Debug)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub action: fn(&mut Shell, Vec<String>) -> Result<(), String>,
}

impl Command {
    /// Run a command given a shell
    pub fn run(&self, shell: &mut Shell, args: Vec<String>) -> Result<(), String> {
        (self.action)(shell, args)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Shell, State};
    use super::Command;

    #[test]
    fn test_command_run() {
        // Create a test shell
        let mut shell = Shell::new();

        fn test_action(shell: &mut Shell, args: Vec<String>) -> Result<(), String> {
            assert_eq!(args, vec!["arg1".to_string(), "arg2".to_string()], "args"); // Assert the args
            assert_eq!(shell.state, State::Ok, "state"); // Assert the state

            Ok(()) // Return Ok if the action succeeds
        }

        // Create a test command
        let command = Command {
            name: "test".to_string(),
            description: "test command".to_string(),
            usage: "test <arg1> <arg2>".to_string(),
            action: test_action,
        };

        // Call the run method and assert the result
        let args = vec!["arg1".to_string(), "arg2".to_string()];
        let result = command.run(&mut shell, args);
        assert_eq!(result, Ok(()));
    }
}
