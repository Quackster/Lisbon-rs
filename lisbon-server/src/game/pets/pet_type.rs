//! Mirrors `net.h4bbo.lisbon.game.pets.PetType`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PetType {
    Dog,
    Cat,
    Croc,
}

impl PetType {
    /// Const array mirroring `values()`.
    pub const ALL: [PetType; 3] = [PetType::Dog, PetType::Cat, PetType::Croc];
}
