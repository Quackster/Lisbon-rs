//! Mirrors `net.h4bbo.lisbon.game.misc.figure.FigureSetType`.

#[derive(Clone, Debug)]
pub struct FigureSetType {
    set: String,
    palette_id: i32,
    is_mandatory: bool,
}

impl FigureSetType {
    /// Mirrors the 3-arg `FigureSetType(String, int, boolean)` constructor.
    pub fn new(set: String, palette_id: i32, is_mandatory: bool) -> Self {
        Self {
            set,
            palette_id,
            is_mandatory,
        }
    }

    /// Mirrors `getSet()`.
    pub fn get_set(&self) -> &str {
        &self.set
    }

    /// Mirrors `getPaletteId()`.
    pub fn get_palette_id(&self) -> i32 {
        self.palette_id
    }

    /// Mirrors `isMandatory()`.
    pub fn is_mandatory(&self) -> bool {
        self.is_mandatory
    }
}
