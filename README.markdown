# LUHNY.RS

![GitHub CI](https://github.com/angeldollface/luhny.rs/actions/workflows/rust.yml/badge.svg)

***My Rustacean implementation of the Luhn algorithm for IMEI numbers.***

## ABOUT

This is my Rustacean implementation of a package I wrote in ECMA Script a couple of months ago. (Link in the section below.) Both these packages do one thing: They provide functions for you to check whether the IMEI number of your smartphone is valid or not. The algorithm used here is the "Luhn" algorithm. Other implementations and web apps showcasing my implementations can be found in the section below. Enjoy.

## LINKS

- Library implementation in ECMA Script: [VIEW](https://github.com/angeldollface/luhny)
- Web app implementation in Vue.js: [VIEW](https://github.com/angeldollface/ceramic)
- Web app implementation in Yew.rs: [VIEW](https://github.com/angeldollface/ceramic.rs)

## INSTALLATION

### IN A RUST PROJECT

To use ***Luhny.rs*** in your Rust project, add this line to your project's dependencies in the project's `Cargo.toml`:

```TOML
luhny = "0.1.0"
```

### AS A COMMAND LINE TOOL

Make sure you have [Rust](https://rust-lang.org) and [Git](https://git-scm.org) installed to run the following command from a terminal session:

```bash
cargo install luhny
```

This command will put the `luhny` executable on your path and install everything properly.

## USAGE

### API

To understand how to use *Luhny.rs* in your project, please refer to the project's [documentation](https://docs.rs/luhny/0.1.0).

### COMMAND LINE

- Get version info:

```bash
luhny -v
# OR
luhny --version
# OR
luhny version
```

- Get helpful info:

```bash
luhny -h
# OR
luhny --help
# OR
luhny help
```

- Test the validity of an IMEI number:

```bash
luhny -i 353879234252633
# OR
luhny --inn 353879234252633
# OR
luhny inn 353879234252633
```

## CHANGELOG

### Version 0.1.0

- Initial release.
- Upload to GitHub.

## NOTE

- *Luhny.rs* by Alexander Abraham a.k.a. *"Angel Dollface"*
- Licensed under the MIT license.