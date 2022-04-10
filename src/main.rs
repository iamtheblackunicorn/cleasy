/*
CLEASY by Alexander Abraham,
a.k.a. "Angeldust Duke" a.k.a. "The Black Unicorn".
Licensed under the MIT license.
*/

use cleasy::*;

fn main(){

    /// Name, versio, and author data.
    let name: String = String::from("Test App");
    let version: String = String::from("1.0.0");
    let author: String = String::from("Alexander Abraham");

    /// Instantiating the "App" struct with the required
    /// data.
    let mut my_app: App = App::new(name, version, author);

    /// Adding a greeting without data. Note the use of "false".
    my_app.add_arg("greet".to_string(), "generic greeting for the user".to_string(), "false".to_string());

    /// Adding a greeting with data. Note the use of "true".
    my_app.add_arg("cgreet".to_string(), "custom greeting for the user".to_string(), "true".to_string());

    /// Checks if the version info was requested
    /// and prints it out.
    if my_app.version_is() == true{
        println!("{}", my_app.version());
    }

    /// Checks if the help info was requested
    /// and prints it out.
    if my_app.help_is() == true {
        println!("{}", my_app.help());
    }

    /// Checks if the "greet" arg was requested
    /// and prints out a generic greeting.
    if my_app.arg_was_used("greet".to_string()) == true{
        println!("Hello World!");
    }

    /// Checks if the "cgreet" arg was requested
    /// and prints out a greeting with the argument's
    /// data.
    if my_app.arg_was_used("cgreet".to_string()) == true{
        let arg_data: String = my_app.get_arg_data("cgreet".to_string());
        println!("Hello, {}!", arg_data);
    }
}
