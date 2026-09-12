//! Mirrors `net.h4bbo.lisbon.game.misc.figure.FigurePart`.

#[derive(Clone, Debug)]
pub struct FigurePart {
    id: String,
    ty: String,
    colorable: bool,
    index: i32,
}

impl FigurePart {
    /// Mirrors the 4-arg `FigurePart(String, String, boolean, int)` constructor.
    pub fn new(id: String, ty: String, colorable: bool, index: i32) -> Self {
        Self {
            id,
            ty,
            colorable,
            index,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Mirrors `getType()`.
    pub fn get_type(&self) -> &str {
        &self.ty
    }

    /// Mirrors `isColorable()`.
    pub fn is_colorable(&self) -> bool {
        self.colorable
    }

    /// Mirrors `getIndex()`.
    pub fn get_index(&self) -> i32 {
        self.index
    }
}
