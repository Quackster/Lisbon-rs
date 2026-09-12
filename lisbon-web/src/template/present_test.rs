//! Mirrors `org.alexdev.http.template.PresentTest`.

use tera::Context;

/// Mirrors `org.alexdev.http.template.PresentTest`.
/// Port note: tera's `Test` trait has no access to the template context, so the Pebble
/// "present" test cannot be registered with tera; the check is exposed directly.
pub struct PresentTest;

impl PresentTest {
    /// Mirrors `apply(Object, ...)`.
    pub fn apply(input: &str, context: &Context) -> bool {
        context.contains_key(input)
    }

    /// Mirrors `getTests()`.
    pub fn get_tests() -> std::collections::HashMap<&'static str, PresentTest> {
        std::collections::HashMap::from([("present", PresentTest)])
    }
}
