# CLEASY

***Making command-line interfaces in Rust easy.***

![GitHub CI](https://github.com/iamtheblackunicorn/cleasy/actions/workflows/rust.yml/badge.svg)

## ABOUT

I've been writing an awful lot of command line applications in Rust lately. I'm sick and tired of manually parsing the options passed to my programs, so I decided to write a library to make my life easier. I know `clap` would have been option but I don't like drama. `clap` is drama I am not too keen to get into right now.

## FEATURES

- Blazingly fast.
- Easy to use, no drama.
- Provides multiple options out of the box.
- Provides a `-h` or `--help` flag out of the box.
- Provides a `-v` or `--version` flag out of the box.

## INSTALLATION

To use ***Cleasy*** in your rust project, add this line to your project's `Cargo.toml`'s `[dependencies]` section:

```TOML
cleasy = { git = "https://github.com/iamtheblackunicorn/cleasy", version = "1.0.0" }
```

To import the library into your project's code, use this line:

```Rust
use cleasy::App;
```

To find out exactly how to use the library, please check out the section below.

## EXAMPLE

An example using all of ***Cleasy's*** APIs can be found in the sample below:

```Rust
/*
CLEASY by Alexander Abraham,
a.k.a. "Angeldust Duke" a.k.a. "The Black Unicorn".
Licensed under the MIT license.
*/

use cleasy::App;

fn main(){

    /// Name, version, and author data.
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

    if my_app.version_is() == true {
        println!("{}", my_app.version());
    }
    else if my_app.help_is() == true {
        println!("{}", my_app.help());
    }
    else if my_app.arg_was_used("greet".to_string()) == true {
        println!("Hello World!");
    }
    else if my_app.arg_was_used("cgreet".to_string()) == true {
        let arg_data: String = my_app.get_arg_data("cgreet".to_string());
        println!("Hello, {}!", arg_data);
    }
    else {
        println!("{}", my_app.help());
    }
}
```

## CHANGELOG

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE

- *Cleasy* by Alexander Abraham a.k.a. *"The Black Unicorn"*
- Licensed under the MIT license.
