//! Mirrors `net.h4bbo.lisbon.game.player.Wardrobe`.

#[derive(Clone, Debug)]
pub struct Wardrobe {
    slot_id: i32,
    sex: String,
    figure: String,
}

impl Wardrobe {
    /// Mirrors the `Wardrobe(int, String, String)` constructor.
    pub fn new(slot_id: i32, sex: &str, figure: &str) -> Self {
        Self {
            slot_id,
            sex: sex.to_string(),
            figure: figure.to_string(),
        }
    }

    /// Mirrors `getSlotId`.
    pub fn get_slot_id(&self) -> i32 {
        self.slot_id
    }

    /// Mirrors `getSex`.
    pub fn get_sex(&self) -> &str {
        &self.sex
    }

    /// Mirrors `getFigure`.
    pub fn get_figure(&self) -> &str {
        &self.figure
    }
}
