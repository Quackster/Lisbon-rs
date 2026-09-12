//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.CARRYITEM`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::currencies::film::FILM;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct CARRYITEM;

impl MessageEvent for CARRYITEM {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        if room_user.get_room().is_none() {
            return Ok(());
        }

        let contents = reader.contents().unwrap_or_default();

        if contents == "20" {
            player.send(&FILM::new(player.get_details()));

            if !contents.is_empty() && contents.chars().all(|c| c.is_ascii_digit()) {
                room_user.carry_item(contents.parse::<i32>().unwrap_or(0), None);
            } else {
                room_user.carry_item(-1, Some(&contents));
            }

            room_user.reset_room_timer();
        }

        Ok(())
    }
}
