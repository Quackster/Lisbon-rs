//! Mirrors `net.h4bbo.lisbon.messages.incoming.register.APPROVENAME`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::player::player::Player;
use crate::messages::outgoing::register::approvenamely::APPROVENAMERELY;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct APPROVENAME;

impl APPROVENAME {
    /// Mirrors `getNameCheckCode(String)`.
    pub fn get_name_check_code(name: &str) -> i32 {
        let mut name_check_code = 0;

        if PlayerDao::get_id(name) > 0 {
            name_check_code = 4;
        } else if name.chars().count() > 16 {
            name_check_code = 1;
        } else if name.is_empty() {
            name_check_code = 2;
        } else if name.contains(' ')
            || !Self::has_allowed_characters(
                &name.to_lowercase(),
                "1234567890qwertyuiopasdfghjklzxcvbnm-+=?!@:.,$",
            )
            || name.to_uppercase().contains("MOD-")
        {
            name_check_code = 3;
        }

        name_check_code
    }

    /// Mirrors `hasAllowedCharacters(String, String)` (the Java null-check is
    /// inapplicable for a `&str`).
    pub fn has_allowed_characters(value: &str, allowed_chars: &str) -> bool {
        for character in value.chars() {
            if !allowed_chars.contains(character) {
                return false;
            }
        }

        true
    }
}

impl MessageEvent for APPROVENAME {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let name = StringUtil::filter_input(&reader.read_string(), true);
        let name_check_code = Self::get_name_check_code(&name);

        player.send(&APPROVENAMERELY::new(name_check_code));

        Ok(())
    }
}
