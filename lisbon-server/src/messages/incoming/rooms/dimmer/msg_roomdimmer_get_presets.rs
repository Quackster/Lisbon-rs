//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.dimmer.MSG_ROOMDIMMER_GET_PRESETS`.
use crate::game::entity::entity::Entity;
use crate::dao::mysql::moodlight_dao::MoodlightDao;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::dimmer::moodlight_presets::MOODLIGHT_PRESETS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MSG_ROOMDIMMER_GET_PRESETS;

impl MessageEvent for MSG_ROOMDIMMER_GET_PRESETS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let Some(mut item) = room.get_item_manager().get_moodlight() else {
            return Ok(());
        };

        if !MoodlightDao::contains_preset(item.get_id()) {
            MoodlightDao::create_presets(item.get_id());
        }

        let (current_preset, presets) = MoodlightDao::get_presets(item.get_id())
            .ok_or_else(|| "MSG_ROOMDIMMER_GET_PRESETS: missing moodlight presets".to_string())?;

        let current_preset_data = presets
            .get((current_preset - 1) as usize)
            .map(|preset| preset.as_str())
            .unwrap_or("");

        let stale = format!("{current_preset},");
        if !item.get_custom_data().contains(stale.as_str()) {
            item.set_custom_data(&format!("1,{current_preset},{current_preset_data}"));
            item.save();
        }

        player.send(&MOODLIGHT_PRESETS::new(current_preset, presets));

        Ok(())
    }
}
