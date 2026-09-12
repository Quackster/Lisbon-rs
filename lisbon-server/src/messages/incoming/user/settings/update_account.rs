//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.settings.UPDATE_ACCOUNT`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::{PlayerManager, RegisterValueData};
use crate::game::player::register::register_data_type::RegisterDataType;
use crate::game::player::register::register_value::RegisterValue;
use crate::messages::outgoing::user::settings::update_account_response::{
    UPDATE_ACCOUNT_RESPONSE, ResponseType,
};
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct UPDATE_ACCOUNT;

impl MessageEvent for UPDATE_ACCOUNT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        // BU@M@Iqwerty123@H@J06.07.1992@C@F123123
        // header - id - old password - birthday - new password
        let mut register_values = std::collections::BTreeMap::new();
        register_values.insert(3, RegisterValue::new("password", RegisterDataType::String));
        register_values.insert(
            13,
            RegisterValue::new("oldpassword", RegisterDataType::String),
        );
        register_values.insert(7, RegisterValue::new("email", RegisterDataType::String));
        register_values.insert(8, RegisterValue::new("birthday", RegisterDataType::String));

        while !reader.remaining_bytes().is_empty() {
            let value_id = reader.read_base64();

            let Some(value) = register_values.get_mut(&value_id) else {
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

        let as_string = |label: &str| {
            manager
                .get_register_value(&register_values, label)
                .and_then(|value| match value {
                    RegisterValueData::String(text) if !text.is_empty() => Some(text),
                    _ => None,
                })
        };

        let birthday = as_string("birthday");
        let old_password = as_string("oldpassword");
        let new_password = as_string("password");
        let email = as_string("email");

        if !StringUtil::is_null_or_empty(Some(player.get_details().get_birthday()))
            && player.get_details().get_birthday() != birthday.as_deref().unwrap_or("")
        {
            player.send(
                &UPDATE_ACCOUNT_RESPONSE::new(ResponseType::IncorrectBirthday),
            );
            return Ok(());
        }

        let Some(old_password) = old_password else {
            return Ok(());
        };

        let name = player.get_details().get_name().to_string();

        if !PlayerDao::login(player.get_details_mut(), &name, &old_password) {
            player.send(
                &UPDATE_ACCOUNT_RESPONSE::new(ResponseType::IncorrectPassword),
            );
            return Ok(());
        }

        if let Some(new_password) = &new_password {
            PlayerDao::save_password(
                player.get_details().get_id(),
                &manager.create_password(new_password),
            );
            player.send(&UPDATE_ACCOUNT_RESPONSE::new(ResponseType::Success));
        }

        if let Some(email) = &email {
            PlayerDao::save_email(player.get_details().get_id(), email);
            player.send(&UPDATE_ACCOUNT_RESPONSE::new(ResponseType::Success));
        }

        if StringUtil::is_null_or_empty(Some(player.get_details().get_birthday())) {
            let birthday = birthday.unwrap_or_default();
            player.get_details_mut().set_birthday(&birthday);
            PlayerDao::save_birthday(player.get_details().get_id(), &birthday);
        }

        Ok(())
    }
}
