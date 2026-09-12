//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.IIM`.
use crate::game::entity::entity::Entity;
use crate::game::games::triggers::battle_ships_trigger::BattleShipsTrigger;
use crate::game::games::triggers::chess_trigger::ChessTrigger;
use crate::game::games::triggers::poker_trigger::PokerTrigger;
use crate::game::games::triggers::tic_tac_toe_trigger::TicTacToeTrigger;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct IIM;

impl MessageEvent for IIM {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let contents = reader.contents().unwrap_or_default();
        let command_args: Vec<&str> = contents.split(' ').collect();

        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        // If we're on a current item and the current item has a valid
        // trigger
        let Some(current_item) = room_user.get_current_item() else {
            return Ok(());
        };

        // If the trigger isn't a game trigger then ignore it (the Java
        // `instanceof GameTrigger` check; the concrete triggers embed
        // the base).
        let Some(trigger) = current_item
            .get_definition()
            .get_interaction_type()
            .and_then(|interaction_type| interaction_type.get_trigger())
        else {
            return Ok(());
        };

        let Some(mut game) = trigger
            .downcast_ref::<ChessTrigger>()
            .and_then(|t| t.get_game_instance(&room_user.get_position()))
            .or_else(|| {
                trigger
                    .downcast_ref::<PokerTrigger>()
                    .and_then(|t| t.get_game_instance(&room_user.get_position()))
            })
            .or_else(|| {
                trigger
                    .downcast_ref::<BattleShipsTrigger>()
                    .and_then(|t| t.get_game_instance(&room_user.get_position()))
            })
            .or_else(|| {
                trigger
                    .downcast_ref::<TicTacToeTrigger>()
                    .and_then(|t| t.get_game_instance(&room_user.get_position()))
            })
        else {
            return Ok(());
        };

        // The Java `ArrayIndexOutOfBounds` when the command is missing.
        if command_args.len() < 2 {
            return Ok(());
        }

        let game_id = command_args[0];
        let command = command_args[1];

        // The Java returns when the game id doesn't match (including a
        // missing one).
        let Some(server_game_id) = game.get_game_id() else {
            return Ok(());
        };
        if game_id != server_game_id {
            return Ok(());
        }

        let mut arguments = contents
            .replace(format!("{game_id} {command}").as_str(), "");
        if arguments.starts_with(' ') {
            arguments = arguments.trim().to_string();
        }

        let args: Vec<String> = arguments.split(' ').map(|arg| arg.to_string()).collect();

        // The Java passes the `Player` directly; the Rust `handleCommand`
        // takes the shared handle.
        let Some(player_arc) =
            PlayerManager::get_instance().get_player_by_id(player.get_details().get_id())
        else {
            return Ok(());
        };

        game.handle_command(&player_arc, &room, &current_item, command, &args);

        Ok(())
    }
}
