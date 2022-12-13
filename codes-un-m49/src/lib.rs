/*!
This package contains an implementation of the [UN
M49](https://unstats.un.org/unsd/methodology/m49) Standard Country or Area
Codes for Statistical Use (Series M, No. 49) specification.

UN M49 or the Standard Country or Area Codes for Statistical Use (Series M,
No. 49) is a standard for area codes used by the United Nations for
statistical purposes, developed and maintained by the United Nations
Statistics Division. Each area code is a 3-digit number which can refer to a
wide variety of geographical and political regions, like a continent and a
country. Codes assigned in the system generally do not change when the country
or area's name changes (unlike ISO 3166-1 alpha-2 or ISO 3166-1 alpha-3), but
instead change when the territorial extent of the country or area changes
significantly, although there have been exceptions to this rule.

# Example

```rust
use codes_un_m49 as M49;

let region = M49::UN_M69_REGION_258;

assert_eq!(region.code(), 258);
assert_eq!(region.name(), "French Polynesia");
assert_eq!(region.kind(), M49::RegionKind::Country);

// feature = "country_codes"
// assert_eq!(parent.country_code(), Some(CountryCode::PF));
// or
// assert_eq!(region.country_code(), Some("PF"));

let parent = region.parent_code().unwrap();
assert_eq!(parent.code(), 61);
assert_eq!(parent.name(), "Polynesia");

let parent = parent.parent_code().unwrap();
assert_eq!(parent.code(), 9);
assert_eq!(parent.name(), "Oceania");

let parent = parent.parent_code().unwrap();
assert_eq!(parent.code(), 1);
assert_eq!(parent.name(), "World");
```

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [RegionClassificationCode] type.
* `country_codes`; if enabled the value returned by the [RegionClassificationCode::country_code]
  method will be an instance of `CountryCode` from the `codes-iso-3166`
  package, otherwise it returns `'static str`.

*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

// ------------------------------------------------------------------------------------------------
//
// The rest of this file is generated by the package build script.
//
// ------------------------------------------------------------------------------------------------

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod kinds;
pub use kinds::RegionKind;