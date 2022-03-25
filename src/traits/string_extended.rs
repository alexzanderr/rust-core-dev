


/// StringExtended
/// a collections of methods that simplify development workflow
pub trait StringExtended {
    /// split_lines -> split a string by backslash n (`\n`)
    fn split_lines(&self) -> Vec<String>;

    fn split_to_vec_string(&self, pattern: &str) -> Vec<String>;

    fn split_to_vec_str(&self, pattern: &str) -> Vec<&str>;


    /// get_char -> get the character at a specified position
    /// since you cant do this: `string[index]`
    /// and the simplest way in rust without this is `string.chars().nth(index).unwrap()`
    /// i decided to create a helper
    fn get_char(&self, index: usize) -> Option<char>;

    /// makes the first letter of the String/str uppercase
    /// doesnt modify in place
    fn capitalize(&self) -> String;
}


impl StringExtended for String {
    fn split_lines(&self) -> Vec<String> {
        self.split("\n").map(|line| line.to_string()).collect()
    }


    fn split_to_vec_string(&self, pattern: &str) -> Vec<String> {
        self.split(pattern).map(|s| s.to_string()).collect()
    }


    fn split_to_vec_str(&self, pattern: &str) -> Vec<&str> {
        self.split(pattern).collect()
    }


    fn get_char(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }


    fn capitalize(&self) -> String {
        self[0..1].to_uppercase() + &self[1..]
    }
}

impl StringExtended for &str {
    fn split_lines(&self) -> Vec<String> {
        self.split("\n").map(|line| line.to_string()).collect()
    }


    fn get_char(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }

    fn split_to_vec_string(&self, pattern: &str) -> Vec<String> {
        self.split(pattern).map(|s| s.to_string()).collect()
    }

    fn split_to_vec_str(&self, pattern: &str) -> Vec<&str> {
        self.split(pattern).collect()
    }


    fn capitalize(&self) -> String {
        self[0..1].to_uppercase() + &self[1..]
    }
}
