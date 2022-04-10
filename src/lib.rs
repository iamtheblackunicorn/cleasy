/*
CLEASY by Alexander Abraham,
a.k.a. "Angeldust Duke" a.k.a. "The Black Unicorn".
Licensed under the MIT license.
*/

use std::env;
use std::collections::HashMap;

/// A public struct for your
/// awesome CLI app!
pub struct App{
    pub name: String,
    pub version: String,
    pub author: String,
    pub args: HashMap<String, Vec<String>>
}

/// Implementing all methods for
/// your app that give you the freedom
/// to choose what you'd like to do.
impl App{
    /// Instantiates the "App" struct.
    pub fn new(name: String, version: String, author: String) -> App {
        let mut args: HashMap<String, Vec<String>> = HashMap::new();
        return App{
            name: name,
            version: version,
            author: author,
            args:args
        };
    }
    /// Checks if "-a" or "--arg" was used.
    pub fn arg_was_used(&self, arg: String) -> bool {
        let mut result: bool = false;
        let args: Vec<String> = env::args().collect();
        let args_clone_one: Vec<String> = args.clone();
        let arg_clone_one: String = arg.clone();
        let arg_clone_two: String = arg_clone_one.clone();
        let arg_first_letter: String = self.clean_split(arg_clone_one, String::from(""))[1].clone();
        let minus_arg: String = format!("-{}", arg_first_letter);
        let minus_minus_arg: String = format!("--{}", arg_clone_two);
        if args.contains(&minus_arg) || args_clone_one.contains(&minus_minus_arg) {
            result = true;
        }
        else {}
        return result;
    }
    /// Adds an argument to the argument pool.
    /// If you'd like to accept data for an argument,
    /// set the "data" flag to either "true" or "false".
    pub fn add_arg(&mut self, name: String, help: String, data: String) {
        let mut attribute_vec: Vec<String> = Vec::new();
        attribute_vec.push(help);
        attribute_vec.push(data);
        self.args.insert(name, attribute_vec);
    }
    /// Retrieves the command line data for an argument.
    pub fn get_arg_data(&self, name: String) -> String {
        let args: Vec<String> = env::args().collect();
        let args_clone_one: Vec<String> = args.clone();
        let args_clone_two: Vec<String> = args_clone_one.clone();
        let args_clone_three: Vec<String> = args_clone_two.clone();
        let args_clone_four: Vec<String> = args_clone_three.clone();
        let name_clone_one: String = name.clone();
        let name_clone_two: String = name_clone_one.clone();
        let name_clone_three: String = name_clone_two.clone();
        let name_clone_four: String = name_clone_three.clone();
        let mut result: String = String::from("");
        let arg_first_letter: String = self.clean_split(name_clone_one, String::from(""))[1].clone();
        let minus_arg: String = format!("-{}", arg_first_letter);
        let minus_arg_clone: String = minus_arg.clone();
        let minus_minus_arg: String = format!("--{}", name_clone_two);
        let minus_minus_arg_clone: String = minus_minus_arg.clone();
        if &self.args[&name_clone_four][1].clone() == "true" && args_clone_one.contains(&minus_arg){
            let name_index: usize = args_clone_three.iter().position(|r| r == &minus_arg_clone).unwrap();
            let next_pos: usize = name_index + 1;
            result = args_clone_four[next_pos].clone();
        }
        else if &self.args[&name_clone_four][1].clone() == "true" && args_clone_two.contains(&minus_minus_arg) {
            let name_index: usize = args_clone_three.iter().position(|r| r == &minus_minus_arg_clone).unwrap();
            let next_pos: usize = name_index + 1;
            result = args_clone_four[next_pos].clone();
        }
        else {}
        return result;
    }
    /// Returns a boolean to tell you whether version
    /// info was requested or not.
    pub fn version_is(&self) -> bool {
        let mut result: bool = false;
        let args: Vec<String> = env::args().collect();
        let arg_len = args.len();
        if arg_len == 2 {
            if args[1].clone() == "--version" || args[1].clone() == "-v" {
                result = true;
            }
            else {}
        }
        else {}
        return result;
    }
    /// Returns a boolean to tell you whether help
    /// info was requested or not.
    pub fn help_is(&self) -> bool {
        let mut result: bool = false;
        let args: Vec<String> = env::args().collect();
        let arg_len = args.len();
        if arg_len == 2 {
            if args[1].clone() == "--help" || args[1].clone() == "-h" {
                result = true;
            }
            else {}
        }
        else {}
        return result;
    }
    /// Returns a string with version info.
    pub fn version(&self) -> String {
        let version_string: String = format!("{} v.{}\nby {}.", &self.name, &self.version, &self.author);
        return version_string;
    }
    /// Helper function to split strings into vectors.
    pub fn clean_split(&self, subject: String, split_char: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for item in subject.split(&split_char) {
            let new_item: String = item.to_string();
            result.push(new_item);
        }
        return result;
    }
    /// Returns a string with help info.
    pub fn help(&self) -> String {
        let mut help_string_vec: Vec<String> = Vec::new();
        for (key,value) in &self.args {
            if value[1].clone() == "true" {
                let key_clone_one: String = key.clone();
                let key_clone_two: String = key_clone_one.clone();
                let first_letter: String = self.clean_split(key_clone_one, String::from(""))[1].clone();
                let command_help: String = format!("-{} --{} DATA", first_letter, key_clone_two);
                help_string_vec.push(command_help);
            }
            else {
                let key_clone_one: String = key.clone();
                let key_clone_two: String = key_clone_one.clone();
                let first_letter: String = self.clean_split(key_clone_one, String::from(""))[1].clone();
                let command_help: String = format!("-{} --{}", first_letter, key_clone_two);
                help_string_vec.push(command_help);
            }
        }
        let help_string = help_string_vec.join("\n");
        return help_string;
    }
}
