//! Mirrors `net.h4bbo.lisbon.messages.incoming.pets.GETPETSTAT`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::pets::petstat::PETSTAT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETPETSTAT;

impl MessageEvent for GETPETSTAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player.get_room_user().and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        let raw = reader.read_string();
        let pet_data: Vec<&str> = raw.split('\u{0004}').collect();

        let Some(pet_id) = pet_data
            .first()
            .and_then(|value| value.parse::<i32>().ok())
        else {
            return Ok(());
        };

        // The Java `getByInstanceId` scans every entity; pets are the only
        // `PETSTAT`-compatible kind, so the pet list is scanned.
        let Some(pet) = room
            .get_entity_manager()
            .get_pets()
            .into_iter()
            .find(|pet| pet.get_room_user_pet().get_instance_id() == pet_id)
        else {
            return Ok(());
        };

        player.send(&PETSTAT::new(pet));

        Ok(())
    }
}
