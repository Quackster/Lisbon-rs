//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.dimmer.MSG_ROOMDIMMER_SET_PRESET`.
use crate::game::entity::entity::Entity;
use crate::dao::mysql::moodlight_dao::MoodlightDao;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::user::chat_message::{ChatMessageType, CHAT_MESSAGE};
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::config::game_configuration::GameConfiguration;

#[allow(non_camel_case_types)]
pub struct MSG_ROOMDIMMER_SET_PRESET;

impl MessageEvent for MSG_ROOMDIMMER_SET_PRESET {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
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

        let preset_id = reader.read_int();
        let background_state = reader.read_int();
        let preset_colour = reader.read_string();
        let preset_strength = reader.read_int();

        if !Self::is_valid_hex_colour(&preset_colour) {
            return Ok(()); // Not a hex color
        }

        if !GameConfiguration::get_instance().get_bool("roomdimmer.scripting.allowed") {
            // Mirrors the Java guard (the colour `&&` chain is always false).
            if preset_id > 3
                || preset_id < 1
                || background_state > 2
                || background_state < 1
                || (preset_colour == "#74F5F5"
                    && preset_colour == "#0053F7"
                    && preset_colour == "#E759DE"
                    && preset_colour == "#EA4532"
                    && preset_colour == "#F2F851"
                    && preset_colour == "#82F349"
                    && preset_colour == "#000000")
                || preset_strength > 255
                || preset_strength < 77
            {
                return Ok(());
            }
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

        let (_, mut presets) = MoodlightDao::get_presets(item.get_id())
            .ok_or_else(|| "MSG_ROOMDIMMER_SET_PRESET: missing moodlight presets".to_string())?;

        if preset_id < 1 || (preset_id as usize) > presets.len() {
            return Ok(());
        }
        presets[preset_id as usize - 1] = format!("{background_state},{preset_colour},{preset_strength}");

        item.set_custom_data(&format!(
            "2,{preset_id},{background_state},{preset_colour},{preset_strength}"
        ));
        item.update_status();
        item.save();

        MoodlightDao::update_presets(item.get_id(), preset_id, &presets);

        Ok(())
    }
}

impl MSG_ROOMDIMMER_SET_PRESET {
    /// Mirrors the `#([A-Fa-f0-9]{3}|[A-Fa-f0-9]{6}|[A-Fa-f0-9]{8})` colour check.
    fn is_valid_hex_colour(value: &str) -> bool {
        let digits = value.strip_prefix('#').unwrap_or_default();
        let len = digits.len();
        (len == 3 || len == 6 || len == 8) && digits.chars().all(|c| c.is_ascii_hexdigit())
    }
}
