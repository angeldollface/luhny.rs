/*
LUHNY.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module itself.
pub mod modules;

/// Declaring and re-exporting
/// the "cli" module.
pub use modules::cli::*;

/// Declaring and re-exporting
/// the "luhny" module.
pub use modules::luhny::*;