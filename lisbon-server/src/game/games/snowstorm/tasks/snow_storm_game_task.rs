//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.tasks.SnowStormGameTask`.
//! Registered with the `RoomTaskManager` by `RoomTaskManager::start_tasks`
//! (300 ms period, the `Tickable` impl delegates to `run`); the Java
//! `try/catch` (exception logging) has no Rust equivalent.
use std::any::Any;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::game_object::GameObject;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::events::snow_storm_avatar_move_event::SnowStormAvatarMoveEvent;
use crate::game::games::snowstorm::events::snow_storm_machine_add_snowball_event::SnowStormMachineAddSnowballEvent;
use crate::game::games::snowstorm::events::snow_storm_machine_move_snowballs_event::SnowStormMachineMoveSnowballsEvent;
use crate::game::games::snowstorm::mapping::snow_storm_pathfinder::SnowStormPathfinder;
use crate::game::games::snowstorm::objects::snow_storm_machine_object::SnowStormMachineObject;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::games::snowstorm::snow_storm_turn::SnowStormTurn;
use crate::game::games::snowstorm::util::snow_storm_future::SnowStormFuture;
use crate::game::pathfinder::position::Position;
use crate::game::pathfinder::rotation::Rotation;
use crate::game::room::room::Room;
use crate::messages::outgoing::games::snowstorm_game_status::SNOWSTORM_GAMESTATUS;

pub struct SnowStormGameTask {
    room: Room,
    game: Arc<SnowStormGame>,
    snow_storm_turn_list: Mutex<Vec<SnowStormTurn>>,
    future_events: Mutex<Vec<SnowStormFuture>>,
    max_game_turns: i32,
}

impl SnowStormGameTask {
    /// Mirrors the `SnowStormGameTask(Room, SnowStormGame)` constructor
    /// (the `CopyOnWriteArrayList`s are `Mutex<Vec>`).
    pub fn new(room: Room, game: Arc<SnowStormGame>) -> Self {
        let task = Self {
            room,
            game,
            snow_storm_turn_list: Mutex::new(Vec::new()),
            future_events: Mutex::new(Vec::new()),
            max_game_turns: 5,
        };

        task.reset_turns();
        task
    }

    /// Mirrors `resetTurns()`.
    fn reset_turns(&self) {
        let mut turns = self.snow_storm_turn_list.lock();
        turns.clear();

        for _ in 0..self.max_game_turns {
            turns.push(SnowStormTurn::new());
        }
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        if self.game.get_active_players().is_empty() {
            return; // Don't send any packets or do any logic checks during when the game is finished
        }

        let teams = self.game.get_teams();

        for team in teams {
            for game_player in team.lock().get_players() {
                let in_room = {
                    let game_player = game_player.lock();
                    let player = game_player.get_player().lock();
                    player
                        .get_room_user()
                        .and_then(|room_user| room_user.get_room())
                        .is_some_and(|room| room.get_id() == self.room.get_id())
                };

                if in_room {
                    {
                        let guard = game_player.lock();
                        let player = guard.get_player().lock();
                        if let Some(room_user) = player.get_room_user() {
                            room_user.handle_spam_ticks();
                        }
                    }
                    self.process_entity(&game_player, &self.game);
                }
            }
        }

        let futures: Vec<SnowStormFuture> =
            std::mem::take(self.future_events.lock().as_mut());

        for future in futures {
            if future.get_frames_future() == 0 {
                self.send_queue(0, future.get_sub_turn(), future.into_event());
            } else {
                self.future_events.lock().push(future);
            }
        }

        // "if (this.snowStormTurnList.stream().anyMatch(turn -> turn.getSubTurns().size() > 0))"
        let turns = std::mem::take(self.snow_storm_turn_list.lock().as_mut());
        let message = SNOWSTORM_GAMESTATUS::new(turns);

        for player in self.game.get_room().get_entity_manager().get_players() {
            player.lock().send(&message);
        }

        self.reset_turns();

        for future in self.future_events.lock().iter_mut() {
            if future.get_frames_future() > 0 {
                future.decrement_frame();
            }
        }
    }

    /// Mirrors `sendQueue(int, int, GameObject)`.
    pub fn send_queue(
        &self,
        frames_future: i32,
        sub_turn: i32,
        snow_storm_event: Arc<dyn crate::game::games::game_object::GameObject>,
    ) {
        if frames_future == 0 {
            // The Java swallows the out-of-bounds `ArrayIndexOutOfBounds`.
            if let Some(turn) = self
                .snow_storm_turn_list
                .lock()
                .get_mut((sub_turn - 1) as usize)
            {
                turn.add_sub_turn(snow_storm_event);
            }
            return;
        }

        self.future_events
            .lock()
            .push(SnowStormFuture::new(frames_future, sub_turn, snow_storm_event));
    }

    /// Process entity.
    /// Mirrors `processEntity(GamePlayer, SnowStormGame)`.
    fn process_entity(
        &self,
        game_player: &Arc<Mutex<GamePlayer>>,
        game: &Arc<SnowStormGame>,
    ) {
        let (walking, goal_coordinates) = {
            let game_player = game_player.lock();
            let attrs = game_player.get_snow_storm_attributes();
            (attrs.is_walking(), attrs.get_goal_world_coordinates())
        };

        if !walking {
            return;
        }

        let (reached, current_position, walk_goal) = {
            let game_player = game_player.lock();
            let attrs = game_player.get_snow_storm_attributes();
            let reached = match (
                attrs.get_current_position(),
                attrs.get_walk_goal(),
            ) {
                (Some(current), Some(goal)) => current == goal,
                // Java `NullPointerException` equivalent.
                _ => false,
            };
            (
                reached,
                attrs.get_current_position().cloned(),
                attrs.get_walk_goal().cloned(),
            )
        };

        if reached {
            let (goal_x, goal_y) = goal_coordinates.map(|c| (c[0], c[1])).unwrap_or((0, 0));
            let object_id = game_player.lock().get_object_id();
            let rotation = match (current_position, walk_goal) {
                (Some(current), Some(goal)) => {
                    Rotation::calculate_walk_direction(&current, &goal)
                }
                // Java `NullPointerException` equivalent.
                _ => 0,
            };

            self.send_queue(
                0,
                1,
                Arc::new(SnowStormAvatarMoveEvent::new(object_id, goal_x, goal_y)),
            );

            let player_arc = Arc::clone(game_player);
            {
                let mut game_player = game_player.lock();
                game_player.get_snow_storm_attributes_mut().set_rotation(rotation);
                game_player.get_snow_storm_attributes_mut().set_next_goal(None);
                game_player.get_snow_storm_attributes_mut().set_walking(false);
            }
            self.try_snowball_machine(&player_arc, game);
            return;
        }

        let next_position = {
            let game_player = game_player.lock();
            let game = &*game;
            SnowStormPathfinder::get_next_direction(&game, &game_player)
        };

        let mut game_player = game_player.lock();

        if let Some(next_position) = next_position {
            let rotation = current_position
                .map(|current| Rotation::calculate_walk_direction(&current, &next_position))
                .unwrap_or(0);

            game_player.get_snow_storm_attributes_mut().set_rotation(rotation);
            game_player
                .get_snow_storm_attributes_mut()
                .set_current_position(Some(next_position.copy()));
            game_player
                .get_snow_storm_attributes_mut()
                .set_next_goal(Some(next_position.copy()));

            let (goal_x, goal_y) = goal_coordinates.map(|c| (c[0], c[1])).unwrap_or((0, 0));
            self.send_queue(
                0,
                1,
                Arc::new(SnowStormAvatarMoveEvent::new(
                    game_player.get_object_id(),
                    goal_x,
                    goal_y,
                )),
            );
        } else {
            game_player.get_snow_storm_attributes_mut().set_next_goal(None);
            game_player.get_snow_storm_attributes_mut().set_walking(false);
        }
    }

    /// Mirrors `trySnowballMachine(GamePlayer, SnowStormGame)`.
    fn try_snowball_machine(
        &self,
        game_player: &Arc<Mutex<GamePlayer>>,
        game: &Arc<SnowStormGame>,
    ) {
        let current_position = game_player
            .lock()
            .get_snow_storm_attributes()
            .get_current_position()
            .cloned();

        // Java `NullPointerException` equivalent.
        let Some(current_position) = current_position else {
            return;
        };

        let snowball_item_position =
            Position::new_xy(current_position.get_x(), current_position.get_y() - 1);

        let map = game.get_map();
        // Java `NullPointerException` equivalent.
        let Some(map) = map else {
            return;
        };

        let tile = map.get_tile(&snowball_item_position);
        // Java `NullPointerException` equivalent.
        let Some(tile) = tile else {
            return;
        };

        let highest_item = tile.get_highest_item();
        let Some(highest_item) = highest_item else {
            return;
        };

        if !highest_item.is_snowball_machine() {
            return;
        }

        let machine = {
            let game_guard = &*game;
            let objects = game_guard.get_objects().lock();
            objects
                .iter()
                .find(|object| {
                    let any: &dyn std::any::Any = object.as_ref();
                    any.downcast_ref::<SnowStormMachineObject>()
                        .is_some_and(|machine| machine.get_position() == snowball_item_position)
                })
                .cloned()
        };

        let Some(machine) = machine else {
            return;
        };

        // The Java `snowMachineAnimation` periodic `FutureRunnable`
        // (`scheduleWithFixedDelay`, every 3 seconds); it cancels itself
        // when the player stops collecting snowballs.
        {
            let machine = Arc::clone(&machine);
            let game = Arc::clone(game);
            let cancel = Arc::new(AtomicBool::new(false));
            thread::spawn(move || {
                while !cancel.load(Ordering::SeqCst) {
                    if !<dyn Any>::downcast_ref::<SnowStormMachineObject>(
                        machine.as_ref() as &dyn Any,
                    )
                    .map(|machine| machine.is_player_collecting_snowballs(&game))
                    .unwrap_or(false)
                    {
                        cancel.store(true, Ordering::SeqCst);
                        return;
                    }

                    if <dyn Any>::downcast_ref::<SnowStormMachineObject>(
                        machine.as_ref() as &dyn Any,
                    )
                    .map(|machine| machine.get_snowballs())
                    .unwrap_or(0)
                        < 5
                    {
                        <dyn Any>::downcast_ref::<SnowStormMachineObject>(
                            machine.as_ref() as &dyn Any,
                        )
                        .map(|machine| machine.increment_snowballs());

                        if let Some(update_task) = game.get_update_task() {
                            update_task.send_queue(
                                0,
                                1,
                                Arc::new(SnowStormMachineAddSnowballEvent::new(
                                    <dyn Any>::downcast_ref::<SnowStormMachineObject>(
                                        machine.as_ref() as &dyn Any,
                                    )
                                    .map(|machine| machine.get_id())
                                    .unwrap_or(0),
                                )),
                            );
                        }
                    }

                    thread::sleep(Duration::from_secs(3));
                }
            });
        }

        // The Java `snowMachineRestock` periodic `FutureRunnable`
        // (`scheduleAtFixedRate`, every second).
        {
            let machine = Arc::clone(&machine);
            let game = Arc::clone(game);
            let game_player = Arc::clone(game_player);
            let cancel = Arc::new(AtomicBool::new(false));
            thread::spawn(move || {
                while !cancel.load(Ordering::SeqCst) {
                    if !<dyn Any>::downcast_ref::<SnowStormMachineObject>(
                        machine.as_ref() as &dyn Any,
                    )
                    .map(|machine| machine.is_player_collecting_snowballs(&game))
                    .unwrap_or(false)
                    {
                        cancel.store(true, Ordering::SeqCst);
                        return;
                    }

                    if <dyn Any>::downcast_ref::<SnowStormMachineObject>(
                        machine.as_ref() as &dyn Any,
                    )
                    .map(|machine| machine.get_snowballs())
                    .unwrap_or(0)
                        > 0
                    {
                        let player_snowballs = game_player
                            .lock()
                            .get_snow_storm_attributes()
                            .get_snowballs();

                        if player_snowballs < 5 {
                            let player_object_id = {
                                let p = game_player.lock();
                                p.get_snow_storm_attributes().increment_snowballs();
                                p.get_object_id()
                            };

                            if let Some(update_task) = game.get_update_task() {
                                update_task.send_queue(
                                    0,
                                    1,
                                    Arc::new(SnowStormMachineMoveSnowballsEvent::new(
                                        player_object_id,
                                        <dyn Any>::downcast_ref::<SnowStormMachineObject>(
                                            machine.as_ref() as &dyn Any,
                                        )
                                        .map(|machine| machine.get_id())
                                        .unwrap_or(0),
                                    )),
                                );
                            }

                            <dyn Any>::downcast_ref::<SnowStormMachineObject>(
                                machine.as_ref() as &dyn Any,
                            )
                            .map(|machine| machine.decrement_snowballs());
                        }
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            });
        }
    }

    /// Mirrors `getExecutingTurns()`.
    // Port note: the Java returns a defensive `ArrayList` copy; the
    // `Box<dyn GameObject>` events cannot be cloned, so the live handle
    // is returned instead.
    pub fn get_executing_turns(&self) -> &Mutex<Vec<SnowStormTurn>> {
        &self.snow_storm_turn_list
    }
}
