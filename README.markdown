# LUHNY.RS :iphone: :lock: :crab:

![GitHub CI](https://github.com/angeldollface/luhny.rs/actions/workflows/rust.yml/badge.svg)

***My Rustacean implementation of the Luhn algorithm for IMEI numbers. :iphone: :lock: :crab:***

## ABOUT :books:

This is my Rustacean implementation of a package I wrote in ECMA Script a couple of weeks ago. (Link in the section below.) Both these packages do one thing: They provide functions for you to check whether the IMEI number of your smartphone is valid or not. The algorithm used here is the "Luhn" algorithm. Other implementations and web apps showcasing my implementations can be found in the section below. Enjoy. :heart:

## LINKS :earth_americas:

- Library implementation in ECMA Script: [VIEW](https://github.com/angeldollface/luhny)
- Web app implementation in Vue.js: [VIEW](https://github.com/angeldollface/ceramic)
- Web app implementation in Yew.rs: [VIEW](https://github.com/angeldollface/ceramic.rs)

## DISCLAIMER :warning:

Currently, the only type of IMEI number supported are those with a length of 15 digits.

## INSTALLATION :inbox_tray:

### IN A RUST PROJECT

To use ***Luhny.rs*** in your Rust project, add this line to your project's dependencies in the project's `Cargo.toml`:

```TOML
luhny = { git = "https://github.com/angeldollface/luhny.rs", version = "1.0.0" }
```

### AS A COMMAND LINE TOOL

Make sure you have [Rust](https://rust-lang.org) and [Git](https://git-scm.org) installed and run the following command from a terminal:

```bash
cargo install --git https://github.com/angeldollface/luhny.rs
```

## USAGE :hammer:

### API

For usage instructions on ***Luhny.rs***'s functions, please check out [`src/lib.rs`](src/lib.rs).
Be sure to follow the installation instructions before using this package.
Code sample:

```Rust
// main.rs
use luhny::*;

fn main() {
  // A fake valid IMEI number.
  let test_IMEI: String = String::from("353879234252633");

  // Should pass!
  assert_eq!(validate_IMEI(&test_IMEI), true);
}
```

### COMMAND LINE

once you have the `luhny` executable on your path, you can run the following commands:

- Get version info:

```bash
luhny -v
# OR
luhny --version
```

- Get helpful info:

```bash
luhny -h
# OR
luhny --help
```

- Test the validity of an IMNEI number:

```bash
luhny -i 353879234252633
# OR
luhny --inn 353879234252633
```

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

### Version 1.1.0

- Added a CLI.
- Modularized everything.
- Removed some functions.
- Added unit tests.
- Added the `coutils` and `cleasy` crates.

## NOTE :scroll:

- *Luhny :iphone: :lock: :crab:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
