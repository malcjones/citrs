use super::cmd;

#[derive(Debug, Clone)]
pub struct Mode {
    pub name: String,
    pub description: String,
    pub commands: Vec<cmd::Command>,
}

pub fn default() -> Vec<Mode> {
    vec![bookmark::mode()]
}

pub mod bookmark {
    use super::{cmd, Mode};

    pub fn mode() -> Mode {
        Mode {
            name: "bookmark".to_string(),
            description: "Bookmark mode".to_string(),
            commands: vec![add(), list()],
        }
    }

    fn add() -> cmd::Command {
        cmd::Command {
            name: "add".to_string(),
            description: "Add a bookmark".to_string(),
            usage: "add <url>".to_string(),
            action: |_, args| {
                if args.is_empty() {
                    return Err("No URL provided".to_string());
                };

                let url = &args[0];
                println!("Bookmarking {}", url);
                Ok(())
            },
        }
    }

    fn list() -> cmd::Command {
        cmd::Command {
            name: "list".to_string(),
            description: "List bookmarks".to_string(),
            usage: "list".to_string(),
            action: |_, _| {
                println!("Listing bookmarks");
                Ok(())
            },
        }
    }
}
