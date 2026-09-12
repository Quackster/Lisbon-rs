//! Mirrors `net.h4bbo.lisbon.game.pets.PetStat`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PetStat {
    Hunger,
    Thirst,
    Happiness,
    Energy,
    Friendship,
}

impl PetStat {
    /// Const array mirroring `values()`.
    pub const ALL: [PetStat; 5] = [
        PetStat::Hunger,
        PetStat::Thirst,
        PetStat::Happiness,
        PetStat::Energy,
        PetStat::Friendship,
    ];

    /// Mirrors `getAttributeType()`.
    pub fn get_attribute_type(&self) -> i32 {
        match self {
            PetStat::Hunger => 6,
            PetStat::Thirst => 3,
            PetStat::Happiness => 6,
            PetStat::Energy => 7,
            PetStat::Friendship => 7,
        }
    }
}
