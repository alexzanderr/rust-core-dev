use std::process::Command;

pub fn get_stdout_in_zsh(command: &str) -> String {
    get_stdout_in_shell("zsh", command)
}

fn get_stdout_in_shell(shell: &str, command: &str) -> String {
    let result =
        Command::new(shell).arg("-c").arg(command).output().unwrap();

    String::from_utf8(result.stdout).unwrap()
}

pub fn get_output_of_command(command: &str) -> String {
    let result =
        Command::new("sh").arg("-c").arg(command).output().unwrap();

    String::from_utf8(result.stdout).unwrap()
}


pub fn run_shell_command(command: &str) -> bool {
    let status = Command::new("sh").arg("-c").arg(command).status();
    match status {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
