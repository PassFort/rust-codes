/*!
This package contains an implementation of the
[ISO-4217](https://www.iso.org/iso-4217-currency-codes.html)
country code specification.

# Example

```rust
use codes_iso_4217::CurrencyCode;

let code = CurrencyCode::BZD;

assert_eq!(code.alpha_code(), "BZD");
assert_eq!(code.numeric_code(), Some(84));
assert_eq!(code.currency_name(), "Belize Dollar");
assert_eq!(code.country_name(), "BELIZE");
assert_eq!(code.is_fund(), false);
assert_eq!(code.is_historical(), false);
assert_eq!(code.withdrawal_date(), None);

assert_eq!(code.standard().title(), "Currency codes");
```

# Features

By default only the `serde` feature is enabled, the [CurrencyCode::alpha_code] and
[CurrencyCode::numeric_code] methods cannot be excluded.

* `serde` - Enables serialization of the [CurrencyCode] type.
* `currency_name` - Adds the [CurrencyCode::currency_name] method.
* `country_name` - Adds the [CurrencyCode::country_name] method.
* `monetary_units` - Adds the [CurrencyCode::monetary_units] method.
* `is_fund` - Adds the [CurrencyCode::is_fund] method.
* `historical_codes` - Adds the [CurrencyCode::is_historical] and [CurrencyCode::withdrawal_date] methods.

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