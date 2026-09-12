//! Mirrors `org.alexdev.http.util.piechart.Slice`.

/// Mirrors `java.awt.Color` (RGBA components, as used by the pie chart).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Mirrors `Color#getRGB()`.
    pub fn rgb(&self) -> u32 {
        ((self.alpha as u32) << 24)
            | ((self.red as u32) << 16)
            | ((self.green as u32) << 8)
            | self.blue as u32
    }
}

/// Mirrors `org.alexdev.http.util.piechart.Slice`.
pub struct Slice {
    label: String,
    value: f64,
    color: Color,
}

impl Slice {
    /// Mirrors `Slice(String, double, Color)`.
    pub fn new(label: impl Into<String>, value: f64, color: Color) -> Self {
        Self {
            label: label.into(),
            value,
            color,
        }
    }

    /// Mirrors `getLabel()`.
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Mirrors `getValue()`.
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Mirrors `getColor()`.
    pub fn get_color(&self) -> &Color {
        &self.color
    }
}
