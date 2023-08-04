/*
LUHNY.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing my library for
/// building CLIs.
use cliply::App;

/// Importing the "validate_imei"
/// method from "./luhny.rs".
use super::luhny::validate_imei;

/// Importing the "is_number_sequence"
/// method from "./luhny.rs".
use super::luhny::is_number_sequence;

/// Luhny's CLI.
pub fn cli() -> () {
   let mut luhny: App = App::new(
        &"Luhny",
        &"1.1.0",
        &"Angel Dollface"
    );
    luhny.add_arg(
        &"inn", 
        &" input IMEI number", 
        &"true"
    );

    // Was the version flag used?
    if luhny.version_is() {
        println!("{}", luhny.version_info());
    }

    // Was the number supplied?
    else if luhny.arg_was_used(&"inn") {
        let number = luhny.get_arg_data(&"inn");
        match number {
            Ok(num) => {
                if is_number_sequence(&num){
                    if validate_imei(&num) {
                        println!("The IMEI number \"{}\" is valid.", &num);
                    }
                    else {
                        println!("The IMEI number \"{}\" is not valid.", &num);
                    }
                }
                else {
                    println!("{}", String::from("No valid number sequence supplied."));
                }
            },
            Err(e) => {
                println!("{}", &e.to_string());
            }
        };
    }

    // Was the help flag used?
    else if luhny.help_is() {
        println!("{}", luhny.help_info());
    }

    // In any other case...
    else {
        println!("{}", luhny.help_info());
    }
}