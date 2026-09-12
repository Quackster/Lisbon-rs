//! Mirrors `net.h4bbo.lisbon.game.games.utils.PowerUpUtil`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::game_scheduler::GameScheduler;
use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::games::battleball::objects::player_update_object::PlayerUpdateObject;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::player::game_player::GamePlayer;

pub struct PowerUpUtil;

impl PowerUpUtil {
    /// Mirrors `stunPlayer(Game, GamePlayer, BattleBallPlayerState)`.
    pub fn stun_player(
        game: Arc<BattleBallGame>,
        game_player: Arc<Mutex<GamePlayer>>,
        state: BattleBallPlayerState,
    ) {
        {
            let mut player = game_player.lock();

            if let Some(room_user) = player.get_player().lock().get_room_user() {
                room_user.stop_walking();
                room_user.set_walking_allowed(false);
            }

            game.get_room().get_mapping().lock().regenerate_collision_map(game.get_room());

            player.set_player_state(state);
        }

        game.add_object_to_queue(Box::new(PlayerUpdateObject::new(
            Arc::clone(&game_player),
        )));

        // Restore the player 5 seconds later.
        GameScheduler::get_instance().schedule(
            {
                let game = Arc::clone(&game);
                let game_player = Arc::clone(&game_player);
                move || {
                    if game.get_game_state() != GameState::Ended {
                        let player = game_player.lock();
                        let player_guard = player.get_player().lock();

                        if let Some(room_user) = player_guard.get_room_user() {
                            room_user.set_walking_allowed(true);
                        }
                    }

                    {
                        let mut player = game_player.lock();
                        player.set_player_state(BattleBallPlayerState::Normal);
                    }

                    game.add_object_to_queue(Box::new(PlayerUpdateObject::new(
                        Arc::clone(&game_player),
                    )));
                }
            },
            5000,
        );
    }
}
