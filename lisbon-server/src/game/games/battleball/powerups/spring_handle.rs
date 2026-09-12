//! Mirrors `net.h4bbo.lisbon.game.games.battleball.powerups.SpringHandle`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::game_scheduler::GameScheduler;
use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::games::battleball::objects::player_update_object::PlayerUpdateObject;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::room::room::Room;

pub struct SpringHandle;

impl SpringHandle {
    /// Mirrors `handle(BattleBallGame, GamePlayer, Room)`.
    pub fn handle(
        game: Arc<BattleBallGame>,
        game_player: Arc<Mutex<GamePlayer>>,
        _room: &Room,
    ) {
        {
            let mut player = game_player.lock();
            player.set_player_state(BattleBallPlayerState::HighJumps);
        }

        game.add_object_to_queue(Box::new(PlayerUpdateObject::new(
            Arc::clone(&game_player),
        )));

        // Restore the player 10 seconds later.
        GameScheduler::get_instance().schedule(
            {
                let game = Arc::clone(&game);
                let game_player = Arc::clone(&game_player);
                move || {
                    if game.get_game_state() == GameState::Ended {
                        return;
                    }

                    if game_player.lock().get_player_state() != BattleBallPlayerState::HighJumps {
                        return;
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
            10000,
        );
    }
}
