//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.dimmer.MSG_ROOMDIMMER_CHANGE_STATE`.
use crate::game::entity::entity::Entity;
use crate::dao::mysql::moodlight_dao::MoodlightDao;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::user::chat_message::{ChatMessageType, CHAT_MESSAGE};
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MSG_ROOMDIMMER_CHANGE_STATE;

impl MessageEvent for MSG_ROOMDIMMER_CHANGE_STATE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if !room.is_owner(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return Ok(());
        }

        let Some(mut item) = room.get_item_manager().get_moodlight() else {
            return Ok(());
        };

        if !MoodlightDao::contains_preset(item.get_id()) {
            MoodlightDao::create_presets(item.get_id());
        }

        if room.get_task_manager().has_task("RainbowTask") {
            room.get_task_manager().cancel_task("RainbowTask");
            player.send(
                &CHAT_MESSAGE::new(
                    ChatMessageType::Whisper,
                    room_user.get_instance_id(),
                    "Rainbow room dimmer cycle has stopped",
                ),
            );
        }

        let (current_preset, presets) = MoodlightDao::get_presets(item.get_id())
            .ok_or_else(|| "MSG_ROOMDIMMER_CHANGE_STATE: missing moodlight presets".to_string())?;

        let is_enabled = item.get_custom_data().chars().next() != Some('2');
        let preset_value = presets
            .get((current_preset - 1) as usize)
            .map(|preset| preset.as_str())
            .unwrap_or("");

        let state = if is_enabled { "2" } else { "1" };
        item.set_custom_data(&format!("{state},{current_preset},{preset_value}"));
        item.update_status();
        item.save();

        Ok(())
    }
}
