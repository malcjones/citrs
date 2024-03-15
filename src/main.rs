use citrs::sh::Shell;
mod defaults;

fn main() {
    let mut shell = Shell::new();
    defaults::populate(&mut shell);
    shell.run();
}
