//! Mirrors `net.h4bbo.lisbon.messages.incoming.register.REGISTER`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::player::player::Player;
use crate::game::player::player_manager::{PlayerManager, RegisterValueData};
use crate::game::player::register::register_data_type::RegisterDataType;
use crate::messages::incoming::register::approvename::APPROVENAME;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct REGISTER;

impl MessageEvent for REGISTER {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, _player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let mut register_values = PlayerManager::get_instance().get_register_values();

        while !reader.remaining_bytes().is_empty() {
            let value_id = reader.read_base64();

            if !register_values.contains_key(&value_id) {
                return Ok(());
            }

            let value = register_values.get_mut(&value_id).unwrap();

            match value.get_data_type() {
                RegisterDataType::String => {
                    value.set_value(&reader.read_string());
                }
                RegisterDataType::Boolean => {
                    value.set_flag(reader.read_bytes(1).first() == Some(&b'A'));
                }
            }
        }

        let get_value = |label: &str| -> String {
            match PlayerManager::get_instance().get_register_value(&register_values, label) {
                Some(RegisterValueData::String(value)) => value,
                _ => String::new(),
            }
        };

        let username = get_value("name");
        let figure = get_value("figure");
        let gender = get_value("sex");
        let email = get_value("email");
        let birthday = get_value("birthday");
        let password = get_value("password");

        if username == password {
            return Ok(());
        }
        if password.len() < 6 {
            return Ok(());
        }
        if password.len() > 10 {
            return Ok(());
        }

        if APPROVENAME::get_name_check_code(&username) > 0 {
            return Ok(());
        }

        let hashed_password = PlayerManager::get_instance().create_password(&password);

        PlayerDao::register(
            &username,
            &hashed_password,
            &figure,
            &gender,
            &email,
            &birthday,
        );

        Ok(())
    }
}
