# Package codes-iana-charset

This package contains an implementation of the
[IANA CHARSET](https://www.iana.org/assignments/character-sets/character-sets.xhtml) registry.

[![crates.io](https://img.shields.io/crates/v/codes-iana-charset.svg)](https://crates.io/crates/codes-iana-charset)
[![docs.rs](https://docs.rs/codes-iana-charset/badge.svg)](https://docs.rs/codes-iana-charset)

These are the official names for character sets that may be used in
the Internet and may be referred to in Internet documentation.  These
names are expressed in ANSI_X3.4-1968 which is commonly called
US-ASCII or simply ASCII.  The character set most commonly use in the
Internet and used especially in protocol standards is US-ASCII, this
is strongly encouraged.  The use of the name US-ASCII is also
encouraged.

The character set names may be up to 40 characters taken from the
printable characters of US-ASCII.  However, no distinction is made
between use of upper and lower case letters.

The MIBenum value is a unique value for use in MIBs to identify coded
character sets.

The value space for MIBenum values has been divided into three
regions. The first region (3-999) consists of coded character sets
that have been standardized by some standard setting organization.
This region is intended for standards that do not have subset
implementations. The second region (1000-1999) is for the Unicode and
ISO/IEC 10646 coded character sets together with a specification of a
(set of) sub-repertoires that may occur.  The third region (>1999) is
intended for vendor specific coded character sets.

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

## Example


```rust
use codes_iana_charset as charset;

let latin_1 = charset::CHARSET_4;
assert_eq!(latin_1.id(), 4);
assert_eq!(latin_1.name(), "ISO_8859-1:1987");
assert_eq!(
    latin_1.source(),
    "[ISO-IR: International Register of Escape Sequences] Note: The current registration authority is IPSJ/ITSCJ, Japan.",
);
assert_eq!(latin_1.preferred_alias(), Some("ISO-8859-1"));
assert_eq!(latin_1.aliases(), &[
    "iso-ir-100",
    "ISO_8859-1",
    "ISO-8859-1",
    "latin1",
    "l1",
    "IBM819",
    "CP819",
    "csISOLatin1"
]);
assert_eq!(latin_1.reference(), Some("[RFC1345][Keld_Simonsen]"));
assert_eq!(latin_1.note(), None);
```
Note that the implementation of `FromStr` takes into account all aliases.

```rust
use codes_iana_charset as charset;
use std::str::FromStr;

let latin_1 = charset::CHARSET_4;

let iso_8859_1 = charset::CharacterSetCode::from_str("ISO_8859-1").unwrap();

assert_eq!(latin_1, iso_8859_1);

let some_charset = charset::CharacterSetCode::try_from(4).unwrap();

assert_eq!(some_charset, iso_8859_1);
```

## Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [CharacterSetCode] type.

## Changes

**Version 0.1.0**

* Initial release

## TODO

TBD