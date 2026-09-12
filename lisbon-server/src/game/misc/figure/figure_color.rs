//! Mirrors `net.h4bbo.lisbon.game.misc.figure.FigureColor`.

#[derive(Clone, Debug)]
pub struct FigureColor {
    colour_id: String,
    index: String,
    is_club_required: bool,
    is_selectable: bool,
}

impl FigureColor {
    /// Mirrors the 4-arg `FigureColor(String, String, boolean, boolean)` constructor.
    pub fn new(
        colour_id: String,
        index: String,
        is_club_required: bool,
        is_selectable: bool,
    ) -> Self {
        Self {
            colour_id,
            index,
            is_club_required,
            is_selectable,
        }
    }

    /// Mirrors `getColourId()`.
    pub fn get_colour_id(&self) -> &str {
        &self.colour_id
    }

    /// Mirrors `getIndex()`.
    pub fn get_index(&self) -> &str {
        &self.index
    }

    /// Mirrors `isClubRequired()`.
    pub fn is_club_required(&self) -> bool {
        self.is_club_required
    }

    /// Mirrors `isSelectable()`.
    pub fn is_selectable(&self) -> bool {
        self.is_selectable
    }
}
