//! Mirrors `net.h4bbo.lisbon.game.infobus.InfobusManager`.
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

use parking_lot::Mutex;

use crate::dao::mysql::infobus_dao::InfobusDao;
use crate::game::entity::entity::Entity;
use crate::game::game_scheduler::GameScheduler;
use crate::game::infobus::infobus_poll::InfobusPoll;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::infobus::cannot_enter_bus::CANNOT_ENTER_BUS;
use crate::messages::outgoing::infobus::poll_question::POLL_QUESTION;
use crate::messages::outgoing::infobus::vote_results::VOTE_RESULTS;
use crate::messages::outgoing::rooms::items::show_program::SHOWPROGRAM;

pub struct InfobusManager {
    can_update_results: AtomicBool,
    is_door_open: AtomicBool,
    is_event_active: AtomicBool,
    current_poll: Mutex<Option<InfobusPoll>>,
    queue_route: Mutex<HashMap<(i32, i32), (i32, i32)>>,
}

impl InfobusManager {
    /// Get the instance.
    pub fn get_instance() -> &'static InfobusManager {
        static INSTANCE: OnceLock<InfobusManager> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    pub fn new() -> Self {
        Self {
            can_update_results: AtomicBool::new(false),
            is_door_open: AtomicBool::new(false),
            is_event_active: AtomicBool::new(false),
            current_poll: Mutex::new(None),
            queue_route: Mutex::new(Self::initialise_queue_route()),
        }
    }

    /// Mirrors `updateDoorStatus(boolean)`.
    pub fn update_door_status(&self, door_status: bool) {
        self.is_door_open.store(door_status, Ordering::SeqCst);

        if let Some(park) = RoomManager::get_instance().get_room_by_model("park_a") {
            for composer in self.get_door_programs() {
                park.lock().send(&composer);
            }
        }
    }

    /// Mirrors `sendDoorStatus(Player)`.
    pub fn send_door_status(&self, player: &Player) {
        for composer in self.get_door_programs() {
            player.send(&composer);
        }
    }

    /// Mirrors `getDoorPrograms()`.
    fn get_door_programs(&self) -> Vec<SHOWPROGRAM> {
        let state = if self.is_door_open() { "open" } else { "close" };

        vec![
            SHOWPROGRAM::new(vec!["bus".to_string(), state.to_string()]),
            SHOWPROGRAM::new(vec!["busDoor".to_string(), state.to_string()]),
        ]
    }

    /// Mirrors `isDoorOpen()`.
    pub fn is_door_open(&self) -> bool {
        self.is_door_open.load(Ordering::SeqCst)
    }

    /// Mirrors `setDoorOpen(boolean)`.
    pub fn set_door_open(&self, door_open: bool) {
        self.is_door_open.store(door_open, Ordering::SeqCst)
    }

    /// Mirrors `isEventActive()`.
    pub fn is_event_active(&self) -> bool {
        self.is_event_active.load(Ordering::SeqCst)
    }

    /// Mirrors `setEventActive(boolean)`.
    pub fn set_event_active(&self, event_active: bool) {
        self.is_event_active.store(event_active, Ordering::SeqCst)
    }

    /// Mirrors `canUpdateResults()`.
    pub fn can_update_results(&self) -> bool {
        self.can_update_results.load(Ordering::SeqCst)
    }

    /// Mirrors `getCurrentPoll()`.
    pub fn get_current_poll(&self) -> Option<InfobusPoll> {
        self.current_poll.lock().clone()
    }

    /// Mirrors `startPolling(int)`.
    pub fn start_polling(&self, poll_id: i32) {
        self.can_update_results.store(false, Ordering::SeqCst);
        *self.current_poll.lock() = InfobusDao::get(poll_id);

        let Some(current_poll) = self.get_current_poll() else {
            return;
        };

        let Some(room) = RoomManager::get_instance().get_room_by_model("park_b") else {
            return;
        };

        for player in room.lock().get_entity_manager().get_players() {
            let player = player.lock();

            if !InfobusDao::has_answer(current_poll.get_id(), player.get_details().get_id()) {
                player.send(&POLL_QUESTION::new(
                    current_poll.get_poll_data().get_question(),
                    current_poll.get_poll_data().get_answers().clone(),
                ));
            }
        }

        GameScheduler::get_instance().schedule(move || {
            Self::get_instance().show_poll_results(poll_id);
        }, 30_000);
    }

    /// Mirrors `showPollResults(int)`.
    pub fn show_poll_results(&self, poll_id: i32) {
        let Some(current_poll) = InfobusDao::get(poll_id) else {
            return;
        };

        self.can_update_results.store(true, Ordering::SeqCst);

        if let Some(room) = RoomManager::get_instance().get_room_by_model("park_b") {
            let answer_results = InfobusDao::get_answers(current_poll.get_id());
            let total_answers: i32 = answer_results.values().sum();

            room.lock().send(&VOTE_RESULTS::new(
                current_poll.get_poll_data().get_question(),
                current_poll.get_poll_data().get_answers().clone(),
                answer_results,
                total_answers,
            ));
        }
    }

    /// Mirrors `stopEvent()`.
    pub fn stop_event(&self) {
        if let Some(room) = RoomManager::get_instance().get_room_by_model("park_b") {
            for player in room.lock().get_entity_manager().get_players() {
                let player = player.lock();

                player.send(&CANNOT_ENTER_BUS::new(
                    "The Infobus event has now ended. Please check the site for updates in future.",
                ));

                if let Some(room_user) = player.get_room_user() {
                    room_user.kick(true);
                }
            }
        }

        self.update_door_status(false);
        *self.current_poll.lock() = None;
    }

    /// Mirrors `getDoorX()`.
    pub fn get_door_x(&self) -> i32 {
        28
    }

    /// Mirrors `getDoorY()`.
    pub fn get_door_y(&self) -> i32 {
        4
    }

    /// Mirrors `getQueueStartX()`.
    pub fn get_queue_start_x(&self) -> i32 {
        19
    }

    /// Mirrors `getQueueStartY()`.
    pub fn get_queue_start_y(&self) -> i32 {
        6
    }

    /// Mirrors `getNextQueueTile(Position)`.
    pub fn get_next_queue_tile(&self, current_position: &Position) -> Option<Position> {
        let route = self.queue_route.lock();
        let tile = route
            .get(&(current_position.get_x(), current_position.get_y()))?;
        Some(Position::new_xy(tile.0, tile.1))
    }

    /// Mirrors `initialiseQueueRoute()`.
    fn initialise_queue_route() -> HashMap<(i32, i32), (i32, i32)> {
        let mut route = HashMap::new();
        route.insert((19, 6), (20, 6));
        route.insert((20, 6), (21, 6));
        route.insert((21, 6), (22, 6));
        route.insert((22, 6), (23, 6));
        route.insert((23, 6), (24, 6));
        route.insert((24, 6), (25, 6));
        route.insert((25, 6), (26, 6));
        route.insert((26, 6), (26, 7));
        route.insert((26, 7), (26, 8));
        route.insert((26, 8), (26, 9));
        route.insert((26, 9), (26, 10));
        route.insert((26, 10), (26, 11));
        route.insert((26, 11), (26, 12));
        route.insert((26, 12), (27, 12));
        route.insert((27, 12), (28, 12));
        route.insert((28, 12), (28, 11));
        route.insert((28, 11), (28, 10));
        route.insert((28, 10), (28, 9));
        route.insert((28, 9), (28, 8));
        route.insert((28, 8), (28, 7));
        route.insert((28, 7), (28, 6));
        route.insert((28, 6), (28, 5));
        route.insert((28, 5), (28, 4));
        route
    }
}
