//! Mirrors `net.h4bbo.lisbon.game.pets.PetAction`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PetAction {
    Talk,
    Sit,
    Lay,
    Play,
    Walk,
}

impl PetAction {
    /// Const array mirroring `values()`.
    pub const ALL: [PetAction; 5] = [
        PetAction::Talk,
        PetAction::Sit,
        PetAction::Lay,
        PetAction::Play,
        PetAction::Walk,
    ];

    /// Mirrors `getActionLength()`.
    pub fn get_action_length(&self) -> i32 {
        0
    }
}
