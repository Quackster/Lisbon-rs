//! Mirrors `net.h4bbo.lisbon.game.misc.figure.FigureSet`.

use crate::game::misc::figure::figure_part::FigurePart;

#[derive(Clone, Debug)]
pub struct FigureSet {
    ty: String,
    id: String,
    gender: String,
    is_club: bool,
    is_colorable: bool,
    is_selectable: bool,
    figure_parts: Vec<FigurePart>,
}

impl FigureSet {
    /// Mirrors the 6-arg `FigureSet(String, String, String, boolean, boolean, boolean)` constructor.
    pub fn new(
        ty: String,
        id: String,
        gender: String,
        is_club: bool,
        is_colorable: bool,
        is_selectable: bool,
    ) -> Self {
        Self {
            ty,
            id,
            gender,
            is_club,
            is_colorable,
            is_selectable,
            figure_parts: Vec::new(),
        }
    }

    /// Mirrors `getType()`.
    pub fn get_type(&self) -> &str {
        &self.ty
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Mirrors `getGender()`.
    pub fn get_gender(&self) -> &str {
        &self.gender
    }

    /// Mirrors `isClub()`.
    pub fn is_club(&self) -> bool {
        self.is_club
    }

    /// Mirrors `isColorable()`.
    pub fn is_colorable(&self) -> bool {
        self.is_colorable
    }

    /// Mirrors `isSelectable()`.
    pub fn is_selectable(&self) -> bool {
        self.is_selectable
    }

    /// Mirrors `getFigureParts()`.
    pub fn get_figure_parts(&self) -> &[FigurePart] {
        &self.figure_parts
    }

    /// Mirrors the direct `List<FigurePart>` mutation of `getFigureParts()`
    /// (used by `FigureManager.loadFigureSets()`).
    pub fn add_figure_part(&mut self, part: FigurePart) {
        self.figure_parts.push(part);
    }
}
