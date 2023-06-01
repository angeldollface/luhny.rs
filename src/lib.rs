/*
LUHNY.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module itself.
mod modules;

/// Declaring and exporting
/// the "cli" module.
pub use modules::cli::*;

/// Declaring and exporting
/// the "tests" module.
#[cfg(test)]
pub use modules::tests::*;

/// Declaring and exporting
/// the "luhny" module.
pub use modules::luhny::*;