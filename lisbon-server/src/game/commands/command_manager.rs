//! Mirrors `net.h4bbo.lisbon.game.commands.CommandManager`.

use std::sync::{Arc, OnceLock};

use crate::game::commands::clientside::chooser_command::ChooserCommand;
use crate::game::commands::clientside::events_command::EventsCommand;
use crate::game::commands::clientside::furni_command::FurniCommand;
use crate::game::commands::command::Command;
use crate::game::commands::registered::about_command::AboutCommand;
use crate::game::commands::registered::afk_command::AfkCommand;
use crate::game::commands::registered::change_motto_command::ChangeMottoCommand;
use crate::game::commands::registered::coords_command::CoordsCommand;
use crate::game::commands::registered::give_badge_command::GiveBadgeCommand;
use crate::game::commands::registered::give_credits_command::GiveCreditsCommand;
use crate::game::commands::registered::give_drink_command::GiveDrinkCommand;
use crate::game::commands::registered::help_command::HelpCommand;
use crate::game::commands::registered::hotel_alert_command::HotelAlertCommand;
use crate::game::commands::registered::packet_test_command::PacketTestCommand;
use crate::game::commands::registered::pick_all_command::PickAllCommand;
use crate::game::commands::registered::poof_command::PoofCommand;
use crate::game::commands::registered::rainbow_dimmer_command::RainbowDimmerCommand;
use crate::game::commands::registered::reload_command::ReloadCommand;
use crate::game::commands::registered::set_config_command::SetConfigCommand;
use crate::game::commands::registered::set_item_price_command::SetItemPriceCommand;
use crate::game::commands::registered::shutdown_command::ShutdownCommand;
use crate::game::commands::registered::sit_command::SitCommand;
use crate::game::commands::registered::talk_command::TalkCommand;
use crate::game::commands::registered::ufos_command::UfosCommand;
use crate::game::commands::registered::uptime_command::UptimeCommand;
use crate::game::commands::registered::users_online_command::UsersOnlineCommand;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuserights_manager::FuserightsManager;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_rank::PlayerRank;
use crate::game::texts::texts_manager::TextsManager;
use crate::messages::outgoing::alert::alert::ALERT;

/// Mirrors `CommandManager`.
pub struct CommandManager {
    commands: Vec<(Vec<String>, Arc<dyn Command>)>,
}

impl CommandManager {
    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        let mut this = Self { commands: Vec::new() };

        this.register(&["help", "commands"], Arc::new(HelpCommand::new()));
        this.register(&["about", "info"], Arc::new(AboutCommand::new()));
        this.register(&["givedrink"], Arc::new(GiveDrinkCommand::new()));
        this.register(&["sit"], Arc::new(SitCommand::new()));
        this.register(&["uptime", "status"], Arc::new(UptimeCommand::new()));
        this.register(&["poof", "update"], Arc::new(PoofCommand::new()));
        this.register(&["coords"], Arc::new(CoordsCommand::new()));
        this.register(&["pickall"], Arc::new(PickAllCommand::new()));
        this.register(
            &["usersonline", "whosonline"],
            Arc::new(UsersOnlineCommand::new()),
        );
        this.register(&["rgb", "rainbow"], Arc::new(RainbowDimmerCommand::new()));
        this.register(&["afk", "idle"], Arc::new(AfkCommand::new()));
        this.register(&["motto"], Arc::new(ChangeMottoCommand::new()));

        // Staff commands
        this.register(&["givebadge"], Arc::new(GiveBadgeCommand::new()));
        this.register(&["packet"], Arc::new(PacketTestCommand::new()));
        this.register(&["reload"], Arc::new(ReloadCommand::new()));
        this.register(&["shutdown"], Arc::new(ShutdownCommand::new()));
        this.register(&["setprice"], Arc::new(SetItemPriceCommand::new()));
        this.register(&["setconfig"], Arc::new(SetConfigCommand::new()));
        this.register(&["hotelalert"], Arc::new(HotelAlertCommand::new()));
        this.register(&["ufos"], Arc::new(UfosCommand::new()));
        this.register(&["talk"], Arc::new(TalkCommand::new()));
        this.register(&["givecredits"], Arc::new(GiveCreditsCommand::new()));

        // Add client-side commands to list
        this.register(&["chooser"], Arc::new(ChooserCommand::new()));
        this.register(&["furni"], Arc::new(FurniCommand::new()));
        this.register(&["events"], Arc::new(EventsCommand::new()));

        // Java: log.info("Loaded {} commands", commands.size());
        let _ = this.commands.len();

        this
    }

    /// Mirrors the `LinkedHashMap` puts.
    fn register(&mut self, aliases: &[&str], command: Arc<dyn Command>) {
        self.commands
            .push((aliases.iter().map(|s| s.to_string()).collect(), command));
    }

    /// Mirrors `getCommand(String)`.
    pub fn get_command(&self, command_name: &str) -> Option<Arc<dyn Command>> {
        for (aliases, command) in &self.commands {
            for name in aliases {
                if command_name.eq_ignore_ascii_case(name) {
                    return Some(command.clone());
                }
            }
        }

        None
    }

    /// Mirrors `hasCommand(Entity, String)`.
    pub fn has_command(&self, entity: &dyn Entity, message: &str) -> bool {
        if message.starts_with(':') && message.len() > 1 {
            let command_name = message
                .split(':')
                .nth(1)
                .unwrap_or("")
                .split(' ')
                .next()
                .unwrap_or("");

            if let Some(cmd) = self.get_command(command_name) {
                return self.has_command_permission(entity, &*cmd);
            }
        }

        false
    }

    /// Mirrors `hasCommandPermission(Entity, Command)`.
    pub fn has_command_permission(&self, entity: &dyn Entity, cmd: &dyn Command) -> bool {
        let permissions = cmd.get_permissions();

        if !permissions.is_empty() {
            for permission in &permissions {
                if entity.has_fuse(permission) {
                    return true;
                }
            }
        } else {
            return true;
        }

        false
    }

    /// Mirrors `hasPermission(PlayerDetails, String)`.
    pub fn has_permission(&self, player_details: &PlayerDetails, command_name: &str) -> bool {
        let Some(cmd) = self.get_command(command_name) else {
            return false;
        };

        let fuserights = FuserightsManager::get_instance()
            .get_fuserights_for_rank(
                player_details
                    .get_rank()
                    .unwrap_or(PlayerRank::Rankless),
            );

        let has_rank = cmd
            .get_permissions()
            .iter()
            .any(|x| fuserights.contains(x));

        if has_rank {
            return true;
        }

        false
    }

    /// Mirrors `invokeCommand(Entity, String)`.
    pub fn invoke_command(&self, entity: &mut dyn Entity, message: &str) {
        let command_name = message
            .split(':')
            .nth(1)
            .unwrap_or("")
            .split(' ')
            .next()
            .unwrap_or("")
            .to_string();
        let cmd = self.get_command(&command_name);

        let args: Vec<String> = if message.len() > command_name.len() + 2 {
            message
                .replace(&format!(":{command_name} "), "")
                .split(' ')
                .map(String::from)
                .collect()
        } else {
            Vec::new()
        };

        if let Some(cmd) = cmd {
            if args.len() < cmd.get_arguments().len() {
                if let Some(player) = entity.as_player() {
                    player.send(
                        &ALERT::new(&TextsManager::get_instance().get_value(
                            "player_commands_no_args",
                        )),
                    );
                }
                return;
            }

            cmd.handle_command(entity, message, &args);
        }
    }

    /// Mirrors `getCommands()`.
    pub fn get_commands(&self) -> Vec<(Vec<String>, Arc<dyn Command>)> {
        self.commands.clone()
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static CommandManager {
        static INSTANCE: OnceLock<Arc<CommandManager>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::new())).as_ref()
    }
}

