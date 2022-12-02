/*!
This package contains an implementation of the
[ISO-10383](https://www.iso.org/standard/61067.html) Securities and related
financial instruments — Codes for exchanges and market identification (MIC)
specification.

ISO 10383 specifies a universal method of identifying exchanges, trading
platforms, regulated or non-regulated markets and trade reporting facilities
as sources of prices and related information in order to facilitate automated
processing.

It is intended for use in any application and communication for identification of places

* where a financial instrument is listed (place of official listing),
* where a related trade is executed (place of trade), and
* where trade details are reported (trade reporting facility).

Note that field descriptions are taken from [ISO 10383 Market Identifier Codes
- Release 2.0
Factsheet](https://www.iso20022.org/sites/default/files/2022-11/ISO10383_MIC_Release_2_0_Factsheet_v2.pdf).

# Example

YYYYY

# Features

By default only the `serde` feature is enabled, the [MarketIdCode::code] and
[MarketIdCode::operating_code], and [MarketIdCode::is_segment] methods cannot be excluded.

* `serde` - Enables serialization of the [CurrencyCode] type.
* `market_name`
* `legal_entity`

* `category_description`

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
mod category;
pub use category::Category;

#[doc(hidden)]
mod status;
pub use status::Status;