

use pretty_assertions::assert_eq;
use rstest::rstest;


#[cfg(test)]
mod index_of_vector_trait {
    use super::assert_eq;
    use super::rstest;

    use core_dev::shell::run_shell_command;
    use core_dev::shell::get_stdout_of_command;

    #[test]
    fn test_get_stdout_of_command() {
        let command = "exa";
        let output = get_stdout_of_command(command);
//         assert_eq!(output, "Cargo.lock
// Cargo.toml
// CONTRIBUTING.md
// docs
// examples
// LICENSE
// Makefile
// README.md
// rust-core-dev.sublime-project
// rust-core-dev.sublime-workspace
// src
// static
// target
// tests
// TODO.md
// ");
    }


    #[test]
    fn test_run_shell_command() {
        let command = "exa";
        let output = run_shell_command(command);
        assert_eq!(output, true);
    }

    use core_dev::shell::get_stdout_in_zsh;

    #[test]
    fn test_run_in_zsh() {
        let command = "echo $ZSH_VERSION";
        let output = get_stdout_in_zsh(command);
        assert_eq!(output, "5.8.1\n");
    }
}
