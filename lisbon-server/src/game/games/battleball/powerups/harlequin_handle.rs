//! Mirrors `net.h4bbo.lisbon.game.games.battleball.powerups.HarlequinHandle`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::game_scheduler::GameScheduler;
use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::games::battleball::objects::player_update_object::PlayerUpdateObject;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::room::room::Room;

pub struct HarlequinHandle;

impl HarlequinHandle {
    /// Mirrors `handle(BattleBallGame, GamePlayer, Room)`.
    pub fn handle(
        game: Arc<BattleBallGame>,
        game_player: Arc<Mutex<GamePlayer>>,
        _room: &Room,
    ) {
        let mut affected_players: Vec<Arc<Mutex<GamePlayer>>> = Vec::new();

        let user_team_id = game_player.lock().get_team_id();

        for p in game.get_active_players() {
            let (colouring_id, team_id, state) = {
                let p_guard = p.lock();
                (
                    p_guard.get_colouring_for_opponent_id(),
                    p_guard.get_team_id(),
                    p_guard.get_player_state(),
                )
            };

            if colouring_id != -1 || team_id == user_team_id {
                continue;
            }

            if state != BattleBallPlayerState::Normal {
                continue // Don't override people using power ups
            }

            {
                let mut p_guard = p.lock();
                p_guard.set_player_state(BattleBallPlayerState::ColouringForOpponent);
                p_guard.set_harlequin_player(Some(Arc::clone(&game_player)));
            }

            game.add_object_to_queue(Box::new(PlayerUpdateObject::new(Arc::clone(&p))));
            affected_players.push(p);
        }

        // Restore the players 10 seconds later.
        GameScheduler::get_instance().schedule(
            {
                let game = Arc::clone(&game);
                let affected_players = affected_players.clone();
                move || {
                    if game.get_game_state() == GameState::Ended {
                        return;
                    }

                    for p in affected_players {
                        {
                            let mut p_guard = p.lock();
                            p_guard.set_player_state(BattleBallPlayerState::Normal);
                            p_guard.set_harlequin_player(None);
                        }

                        game.add_object_to_queue(Box::new(PlayerUpdateObject::new(
                            Arc::clone(&p),
                        )));
                    }
                }
            },
            10000,
        );
    }
}
