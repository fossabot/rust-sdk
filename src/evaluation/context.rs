use std::collections::HashMap;

use typed_builder::TypedBuilder;

use crate::EvaluationContextFieldValue;

/// The evaluation context provides ambient information for the purposes of flag evaluation.
/// Contextual data may be used as the basis for targeting, including rule-based evaluation,
/// overrides for specific subjects, or fractional flag evaluation.
///
/// The context might contain information about the end-user, the application, the host, or any
/// other ambient data that might be useful in flag evaluation. For example, a flag system might
/// define rules that return a specific value based on the user's email address, locale, or the
/// time of day. The context provides this information. The context can be optionally provided at
/// evaluation, and mutated in before hooks.
#[derive(Clone, TypedBuilder, Default, PartialEq, Debug)]
pub struct EvaluationContext {
    /// The targeting key uniquely identifies the subject (end-user, or client service) of a flag
    /// evaluation. Providers may require this field for fractional flag evaluation, rules, or
    /// overrides targeting specific users. Such providers may behave unpredictably if a targeting
    /// key is not specified at flag resolution.
    #[builder(default, setter(into, strip_option))]
    pub targeting_key: Option<String>,

    /// The evaluation context MUST support the inclusion of custom fields, having keys of type
    /// string, and values of type boolean | string | number | datetime | structure.
    #[builder(default)]
    pub custom_fields: HashMap<String, EvaluationContextFieldValue>,
}

impl EvaluationContext {
    /// Add `key` and `value` to the custom field of evaluation context.
    #[must_use]
    pub fn with_custom_field(
        mut self,
        key: impl Into<String>,
        value: impl Into<EvaluationContextFieldValue>,
    ) -> Self {
        self.add_custom_field(key, value);
        self
    }

    /// Add `key` and `value` to the custom field of evaluation context.
    pub fn add_custom_field(
        &mut self,
        key: impl Into<String>,
        value: impl Into<EvaluationContextFieldValue>,
    ) {
        self.custom_fields.insert(key.into(), value.into());
    }

    /// Merge `other` into `self` if corresponding field is not set.
    /// Meaning values set into `self` has higher precedence.
    pub fn merge_missing(&mut self, other: &Self) {
        if self.targeting_key.is_none() {
            if let Some(targeting_key) = &other.targeting_key {
                self.targeting_key = Some(targeting_key.clone());
            }
        }

        other.custom_fields.iter().for_each(|(key, value)| {
            if !self.custom_fields.contains_key(key) {
                self.custom_fields.insert(key.clone(), value.clone());
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use spec::spec;
    use time::OffsetDateTime;

    use super::*;

    #[test]
    fn merge_missig_given_empty() {
        let mut context = EvaluationContext::builder()
            .targeting_key("Targeting Key")
            .build()
            .with_custom_field("Some", "Value");

        let expected = context.clone();

        context.merge_missing(&EvaluationContext::default());

        assert_eq!(context, expected);
    }

    #[test]
    fn merge_missing_given_targeting_key() {
        let mut context = EvaluationContext::builder()
            .targeting_key("Targeting Key")
            .build();

        let expected = context.clone();

        context.merge_missing(
            &EvaluationContext::builder()
                .targeting_key("Another Key")
                .build(),
        );

        assert_eq!(context, expected);
    }

    #[test]
    fn merge_missing_given_custom_fields() {
        let mut context = EvaluationContext::builder()
            .targeting_key("Targeting Key")
            .build()
            .with_custom_field("Key", "Value");

        context.merge_missing(
            &EvaluationContext::default()
                .with_custom_field("Key", "Another Value")
                .with_custom_field("Another Key", "Value"),
        );

        assert_eq!(
            context,
            EvaluationContext::builder()
                .targeting_key("Targeting Key")
                .build()
                .with_custom_field("Key", "Value")
                .with_custom_field("Another Key", "Value")
        )
    }

    #[test]
    fn merge_missing_given_full() {
        let mut context = EvaluationContext::default();

        let other = EvaluationContext::builder()
            .targeting_key("Targeting Key")
            .build()
            .with_custom_field("Key", "Value");

        context.merge_missing(&other);

        assert_eq!(context, other);
    }

    #[derive(Clone, PartialEq, Eq, TypedBuilder, Debug)]
    pub struct DummyStruct {
        pub id: i64,

        #[builder(setter(into))]
        pub name: String,
    }

    #[spec(
        number = "3.1.1",
        text = "The evaluation context structure MUST define an optional targeting key field of type string, identifying the subject of the flag evaluation."
    )]
    #[spec(
        number = "3.1.2",
        text = "The evaluation context MUST support the inclusion of custom fields, having keys of type string, and values of type boolean | string | number | datetime | structure."
    )]
    #[spec(
        number = "3.1.3",
        text = "The evaluation context MUST support fetching the custom fields by key and also fetching all key value pairs."
    )]
    #[spec(
        number = "3.1.4",
        text = "The evaluation context fields MUST have an unique key."
    )]
    #[test]
    fn fields_access() {
        let now_time = OffsetDateTime::now_utc();
        let struct_value = DummyStruct::builder().id(200).name("Bob").build();

        let context = EvaluationContext::builder()
            .targeting_key("Key")
            .build()
            .with_custom_field("Bool", true)
            .with_custom_field("Int", 100)
            .with_custom_field("Float", 3.14)
            .with_custom_field("String", "Hello")
            .with_custom_field("Datetime", now_time)
            .with_custom_field(
                "Struct",
                EvaluationContextFieldValue::Struct(Arc::new(struct_value.clone())),
            );

        assert_eq!(context.targeting_key, Some("Key".to_string()));
        assert_eq!(
            context.custom_fields.get("Int"),
            Some(&EvaluationContextFieldValue::Int(100))
        );
        assert_eq!(
            context.custom_fields.get("Float"),
            Some(&EvaluationContextFieldValue::Float(3.14))
        );
        assert_eq!(
            context.custom_fields.get("String"),
            Some(&EvaluationContextFieldValue::String("Hello".to_string()))
        );
        assert_eq!(
            context.custom_fields.get("Datetime"),
            Some(&EvaluationContextFieldValue::DateTime(now_time))
        );
        assert_eq!(
            *context
                .custom_fields
                .get("Struct")
                .unwrap()
                .as_struct()
                .unwrap()
                .downcast::<DummyStruct>()
                .unwrap(),
            struct_value
        );
    }
}
