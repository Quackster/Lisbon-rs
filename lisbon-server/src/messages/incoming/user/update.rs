//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.UPDATE`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::{PlayerManager, RegisterValueData};
use crate::game::player::register::register_data_type::RegisterDataType;
use crate::game::player::register::register_value::RegisterValue;
use crate::messages::incoming::user::get_info::GET_INFO;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct UPDATE;

impl MessageEvent for UPDATE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        if !player.is_logged_in() {
            return Ok(());
        }

        let register_values = PlayerManager::get_instance().get_register_values();

        let mut values = register_values.clone();

        while !reader.remaining_bytes().is_empty() {
            let value_id = reader.read_base64();

            let Some(value) = values.get_mut(&value_id) else {
                return Ok(());
            };

            match value.get_data_type() {
                RegisterDataType::String => {
                    value.set_value(&reader.read_string());
                }
                RegisterDataType::Boolean => {
                    value.set_flag(reader.read_bytes(1).first() == Some(&b'A'));
                }
            }
        }

        let manager = PlayerManager::get_instance();

        if let Some(RegisterValueData::Boolean(direct_mail)) =
            manager.get_register_value(&values, "directMail")
        {
            player.get_details_mut().set_receive_news(direct_mail);
            PlayerDao::save_receive_mail(player.get_details());
        }

        if let Some(RegisterValueData::String(motto)) =
            manager.get_register_value(&values, "customData")
        {
            player.get_details_mut().set_motto(&motto);
        }

        if let Some(RegisterValueData::String(figure)) =
            manager.get_register_value(&values, "figure")
        {
            player.get_details_mut().set_figure(&figure);
        }

        if let Some(RegisterValueData::String(sex)) =
            manager.get_register_value(&values, "sex")
        {
            player.get_details_mut().set_sex(&sex);
        }

        PlayerDao::save_details(
            player.get_details().get_id(),
            player.get_details().get_figure(),
            player.get_details().get_pool_figure(),
            player.get_details().get_sex(),
        );

        PlayerDao::save_motto(
            player.get_details().get_id(),
            player.get_details().get_motto(),
        );

        let mut empty_reader = NettyRequest::new(Vec::new());
        GET_INFO.handle(player, &mut empty_reader)?;

        if let Some(room_user) = player.get_room_user() {
            room_user.refresh_appearance();
        }

        Ok(())
    }
}
