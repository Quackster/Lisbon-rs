//! Mirrors `net.h4bbo.lisbon.messages.outgoing.pets.PETSTAT`.
use crate::game::pets::pet::Pet;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct PETSTAT {
    pet: Pet,
}

impl PETSTAT {
    /// Mirrors the `PETSTAT(Pet)` constructor.
    pub fn new(pet: Pet) -> Self {
        Self { pet }
    }
}

impl MessageComposer for PETSTAT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.pet.get_room_user_pet().get_instance_id());
        response.write_int(self.pet.get_age());
        response.write_int(self.pet.get_hunger());
        response.write_int(self.pet.get_thirst());
        response.write_int(self.pet.get_happiness());
        response.write_int(self.pet.get_pet_details().get_nature_negative());
        response.write_int(self.pet.get_pet_details().get_nature_positive());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        210 // "CR"
    }
}
