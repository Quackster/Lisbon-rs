//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.settings.CREATEFLAT`.
use crate::dao::mysql::navigator_dao::NavigatorDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::room::models::room_model_manager::RoomModelManager;
use crate::game::texts::texts_manager::TextsManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::rooms::settings::goto_flat::GOTO_FLAT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct CREATEFLAT;

impl MessageEvent for CREATEFLAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let contents = reader.contents().unwrap_or_default();
        let data: Vec<&str> = contents.split('/').collect();

        let floor_setting = data.get(1).copied().unwrap_or_default();
        let room_name = StringUtil::filter_input(
            data.get(2).copied().unwrap_or_default(),
            true,
        );
        let room_model = data.get(3).copied().unwrap_or_default();
        let room_access = data.get(4).copied().unwrap_or_default();
        let room_show_name = data
            .get(5)
            .and_then(|value| value.parse::<i32>().ok())
            .map(|value| value == 1)
            .unwrap_or(false);

        if room_name.replace(' ', "").is_empty() {
            player.send(&ALERT::new(
                &TextsManager::get_instance().get_value("roomatic_givename"),
            ));
            return Ok(());
        }

        if floor_setting != "first floor" {
            return Ok(());
        }

        if !room_model.starts_with("model_") {
            return Ok(());
        }

        if RoomModelManager::get_instance().get_model(&room_model).is_none() {
            return Ok(());
        }

        let model_type = room_model.strip_prefix("model_").unwrap_or_default();

        if model_type != "a" && model_type != "b" && model_type != "c" && model_type != "d"
            && model_type != "e" && model_type != "f" && model_type != "i"
            && model_type != "j" && model_type != "k" && model_type != "l"
            && model_type != "m" && model_type != "n"
            && !player.has_fuse(&Fuseright::UseSpecialRoomLayouts)
        {
            return Ok(()); // Fuck off, scripter.
        }

        let mut access_type: i32 = 0;

        if room_access == "password" {
            access_type = 2;
        }

        if room_access == "closed" {
            access_type = 1;
        }

        let room_id = NavigatorDao::create_room(
            player.get_details().get_id(),
            &room_name,
            &room_model,
            room_show_name,
            access_type,
        );

        if let Some(room_user) = player.get_room_user() {
            room_user.set_authenticate_id(room_id);
        }

        player.send(&GOTO_FLAT::new(room_id, &room_name));

        Ok(())
    }
}
