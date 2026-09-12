//! Mirrors `net.h4bbo.lisbon.game.commands.registered.GiveCreditsCommand`.

use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::rooms::user::chat_message::{ChatMessageType, CHAT_MESSAGE};
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;

/// Mirrors `GiveCreditsCommand`.
pub struct GiveCreditsCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for GiveCreditsCommand {
    /// Mirrors the no-arg constructor.
    fn new() -> Self {
        let mut this = Self {
            permissions: Vec::new(),
            arguments: Vec::new(),
        };
        this.add_permissions();
        this.add_arguments();
        this
    }

    /// Mirrors `addPermissions()`.
    fn add_permissions(&mut self) {
        self.permissions.push(Fuseright::AdministratorAccess);
    }

    /// Mirrors `addArguments()`.
    fn add_arguments(&mut self) {
        self.arguments.push("user".to_string());
        self.arguments.push("credits".to_string());
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
        // :credits Patrick 300

        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let Some(room_user) = player.get_room_user() else {
            return;
        };
        if room_user.get_room().is_none() {
            return;
        }

        let Some(target_user) = PlayerManager::get_instance().get_player_by_name(&args[0]) else {
            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                &format!("Could not find user: {}", args[0]),
            ));
            return;
        };

        if args.len() == 1 {
            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                "Credit amount not provided",
            ));
            return;
        }

        let credits = &args[1];

        // credits should be numeric
        if credits.parse::<i64>().is_err() {
            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                "Credit amount is not a number.",
            ));
            return;
        }

        let target_user = target_user.lock();

        CurrencyDao::increase_credits(
            target_user.get_details(),
            credits.parse::<i32>().unwrap_or(0),
        );

        target_user.send(&CREDIT_BALANCE::new(target_user.get_details().get_credits()));

        player.send(&CHAT_MESSAGE::new(
            ChatMessageType::Whisper,
            room_user.get_instance_id(),
            &format!(
                "{} has been added to user {}",
                credits,
                target_user.get_details().get_name()
            ),
        ));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Give credits to user".to_string()
    }

    /// Mirrors `getPermissions()`.
    fn get_permissions(&self) -> Vec<Fuseright> {
        self.permissions.clone()
    }

    /// Mirrors `getArguments()`.
    fn get_arguments(&self) -> Vec<String> {
        self.arguments.clone()
    }
}
