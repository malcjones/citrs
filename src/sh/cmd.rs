use super::Shell;

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub action: fn(&mut Shell, Vec<String>) -> Result<(), String>
}

impl Command {
    pub fn run(&self, shell: &mut Shell, args: Vec<String>) -> Result<(), String> {
        (self.action)(shell, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::State;

    #[test]
    fn test_command_run() {
        // Create a test shell
        let mut shell = Shell::new();

        fn test_action(shell: &mut Shell, args: Vec<String>) -> Result<(), String> {
            assert_eq!(args, vec!["arg1".to_string(), "arg2".to_string()]); // Assert the arguments
            assert_eq!(shell.state, State::Ok); // Assert the shell state

            Ok(()) // Return Ok if the action succeeds
        }

        // Create a test command
        let command = Command {
            name: "test",
            description: "Test command",
            usage: "test <arg>",
            action: test_action,
        };

        // Call the run method and assert the result
        let args = vec!["arg1".to_string(), "arg2".to_string()];
        let result = command.run(&mut shell, args);
        assert_eq!(result, Ok(()));
    }
}
