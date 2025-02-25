use typed_builder::TypedBuilder;

use crate::{EvaluationReason, FlagMetadata};

/// A structure which contains a subset of the fields defined in the evaluation details,
/// representing the result of the provider's flag resolution process.
#[derive(Clone, TypedBuilder, Debug)]
pub struct ResolutionDetails<T> {
    /// In cases of normal execution, the provider MUST populate the resolution details structure's
    /// value field with the resolved flag value.
    pub value: T,

    /// In cases of normal execution, the provider SHOULD populate the resolution details
    /// structure's variant field with a string identifier corresponding to the returned flag
    /// value.
    #[builder(default, setter(strip_option))]
    pub variant: Option<String>,

    /// The provider SHOULD populate the resolution details structure's reason field with "STATIC",
    /// "DEFAULT", "TARGETING_MATCH", "SPLIT", "CACHED", "DISABLED", "UNKNOWN", "ERROR" or some
    /// other string indicating the semantic reason for the returned flag value.
    #[builder(default, setter(strip_option))]
    pub reason: Option<EvaluationReason>,

    /// The provider SHOULD populate the resolution details structure's flag metadata field.
    #[builder(default, setter(strip_option))]
    pub flag_metadata: Option<FlagMetadata>,
}

impl<T: Default> Default for ResolutionDetails<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            variant: None,
            reason: None,
            flag_metadata: None,
        }
    }
}

impl<T> ResolutionDetails<T> {
    /// Create an instance given value.
    pub fn new<V: Into<T>>(value: V) -> Self {
        Self {
            value: value.into(),
            variant: None,
            reason: None,
            flag_metadata: None,
        }
    }
}
