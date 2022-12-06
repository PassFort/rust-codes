#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A language identifier (also known as a language code or language code
/// element) represents one or more language names, all of which designate the
/// same specific language. The ultimate objects of identification are
/// languages themselves; language names are the formal means by which the
/// languages denoted by language identifiers are designated.
///
///Languages are not static objects; there is variation temporally, spacially,
/// and socially; every language corresponds to some range of variation in
/// linguistic expression. In this part of ISO 639, then, a language
/// identifier denotes some range of language varieties. The scope of
/// languages in Part 3 of ISO 639 is either individual language or
/// macrolanguage. Parts 2 and 5 of ISO 639 also contain codes whose scope is
/// collection of languages.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum LanguageScope {
    /// Individual languages
    Individual,
    /// Macrolanguages
    Macro,
    /// Collections of languages
    Collection,
    /// Dialects
    Dialect,
    /// Reserved for local use
    Reserved,
    /// Special code elements
    Special,
}

/// In the code table for ISO 639-3, the individual languages are identified
/// as being of one of the following five types.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum LanguageType {
    /// Living languages
    ///
    /// A language is listed as living when there are people still living who
    /// learned it as a first language. This part of ISO 639 also includes
    /// identifiers for languages that are no longer living.
    Living,
    /// Extinct languages
    ///
    /// A language is listed as extinct if it has gone extinct in recent
    /// times. (e.g. in the last few centuries). The criteria for identifying
    /// distinct languages in these case are based on intelligibility (as
    /// defined for individual languages).
    Extinct,
    /// Ancient languages
    ///
    /// A language is listed as ancient if it went extinct in ancient times
    /// (e.g. more than a millennium ago). Identifiers are assigned to ancient
    /// languages which have a distinct literature and are treated distinctly
    /// by the scholarly community. It would be ideal to be able to assign
    /// identifiers to ancient languages on the basis of intelligibility, but
    /// ancient records rarely contain enough information to make this
    /// possible. In order to qualify for inclusion in ISO 639-3, the language
    /// must have an attested literature or be well-documented as a language
    /// known to have been spoken by some particular community at some point
    /// in history; it may not be a reconstructed language inferred from
    /// historical-comparative analysis.
    ///
    Ancient,
    /// Historic languages
    ///
    /// A language is listed as historic when it is considered to be distinct
    /// from any modern languages that are descended from it: for instance,
    /// Old English and Middle English. In these cases, the language did not
    /// become extinct; rather, it changed into a different language over
    /// time. Here, too, the criterion is that the language have a literature
    /// that is treated distinctly by the scholarly community.
    ///
    Historic,
    /// Constructed languages
    ///
    /// This part of ISO 639 also includes identifiers that denote constructed
    /// (or artificial) languages. In order to qualify for inclusion the
    /// language must have a literature and it must be designed for the
    /// purpose of human communication. It must be a complete language, and be
    /// in use for human communication by some community long enough to be
    /// passed to a second generation of users. Specifically excluded are
    /// reconstructed languages and computer programming languages.
    Constructed,
    /// Shhhhhhh....
    Special,
}

// ------------------------------------------------------------------------------------------------
//
// The rest of this file is generated by the package build script.
//
// ------------------------------------------------------------------------------------------------

include!(concat!(env!("OUT_DIR"), "/part_3.rs"));
