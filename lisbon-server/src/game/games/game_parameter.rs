//! Mirrors `net.h4bbo.lisbon.game.games.GameParameter`.

#[derive(Clone, Debug)]
pub struct GameParameter {
    name: String,
    editable: bool,
    has_min_max: bool,
    min: i32,
    max: i32,
    default_value: String,
}

impl GameParameter {
    /// Mirrors the 3-arg `GameParameter(String, boolean, String)`
    /// constructor.
    pub fn new(name: &str, editable: bool, default_value: &str) -> Self {
        Self {
            name: name.to_string(),
            editable,
            has_min_max: false,
            min: 0,
            max: 0,
            default_value: default_value.to_string(),
        }
    }

    /// Mirrors the 5-arg `GameParameter(String, boolean, String, int, int)`
    /// constructor.
    pub fn with_min_max(
        name: &str,
        editable: bool,
        default_value: &str,
        min: i32,
        max: i32,
    ) -> Self {
        Self {
            name: name.to_string(),
            editable,
            has_min_max: true,
            min,
            max,
            default_value: default_value.to_string(),
        }
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `isEditable()`.
    pub fn is_editable(&self) -> bool {
        self.editable
    }

    /// Mirrors `hasMinMax()`.
    pub fn has_min_max(&self) -> bool {
        self.has_min_max
    }

    /// Mirrors `getMin()`.
    pub fn get_min(&self) -> i32 {
        self.min
    }

    /// Mirrors `getMax()`.
    pub fn get_max(&self) -> i32 {
        self.max
    }

    /// Mirrors `getDefaultValue()`.
    pub fn get_default_value(&self) -> &str {
        &self.default_value
    }
}
