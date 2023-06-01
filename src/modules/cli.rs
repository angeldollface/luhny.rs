/*
LUHNY.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing my library for
/// building CLIs.
use cleasy::App;

/// Importing the "validate_IMEI"
/// method from "./luhny.rs".
use super::luhny::validate_IMEI;

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
        println!("{}", luhny.version());
    }

    // Was the number supplied?
    else if luhny.arg_was_used(&"inn") == true {
        let number: String = luhny.get_arg_data(&"inn");
        if is_number_sequence(&number){
            if validate_IMEI(&number) {
                println!("The IMEI number \"{}\" is valid.", &number);
            }
            else {
                println!("The IMEI number \"{}\" is not valid.", &number);
            }
        }
        else {
            println!("{}", String::from("No valid number sequence supplied."));
        }
    }

    // Was the help flag used?
    else if luhny.help_is() {
        println!("{}", luhny.help());
    }

    // In any other case...
    else {
        println!("{}", luhny.help());
    }
}