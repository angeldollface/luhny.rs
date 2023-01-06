# LUHNY.RS :iphone: :lock: :crab:

![GitHub CI](https://github.com/angeldollface/luhny.rs/actions/workflows/rust.yml/badge.svg)

***My Rustacean implementation of the Luhn algorithm for IMEI numbers. :iphone: :lock: :crab:***

## ABOUT :books:

This is my Rustacean implementation of a package I wrote in [ECMA Script](https://github.com/angeldollface/luhny) a couple of weeks ago. These packages all do one thing: They provide functions for you to check whether the IMEI number of your smartphone is valid or not. The algorithm used here is the "Luhn" algorithm. Enjoy. :heart:

## DISCLAIMER :warning:

Currently, the only type of IMEI number supported are those with a length of 15 digits.

## INSTALLATION :inbox_tray:

To use ***Luhny.rs*** in your Rust project, add this line to your project's dependencies in the project's `Cargo.toml`:

```TOML
luhny = { git = "https://github.com/angeldollface/luhny.rs", version = "1.0.0" }
```

## USAGE :hammer:

For usage instuctions on ***Luhny.rs***'s functions, please check out [`src/lib.rs`](src/lib.rs).

## LINKS :heart_on_fire:

There are other implementations of my algorithm in other languages:

- ECMA Script: [VIEW](https://github.com/angeldollface/luhny)
- A small web app showcasing the algorithm: [VIEW](https://github.com/angeldollface/ceramic)

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Luhny :iphone: :lock: :crab:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
