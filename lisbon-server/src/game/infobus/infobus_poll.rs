//! Mirrors `net.h4bbo.lisbon.game.infobus.InfobusPoll`.

use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::infobus::infobus_poll_data::InfobusPollData;
use crate::util::date_util::{DateUtil, LONG_DATE};

#[derive(Clone, Debug, serde::Serialize)]
pub struct InfobusPoll {
    id: i32,
    initiated_by: i32,
    infobus_poll_data: InfobusPollData,
    created_at: i64,
}

impl InfobusPoll {
    /// Mirrors the 4-arg `InfobusPoll` constructor.
    pub fn new(id: i32, initiated_by: i32, infobus_poll_data: InfobusPollData, created_at: i64) -> Self {
        Self {
            id,
            initiated_by,
            infobus_poll_data,
            created_at,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getInitiatedBy()`.
    pub fn get_initiated_by(&self) -> i32 {
        self.initiated_by
    }

    /// Mirrors `getCreator()`.
    pub fn get_creator(&self) -> String {
        PlayerDao::get_name(self.initiated_by).unwrap_or_default()
    }

    /// Mirrors `getPollData()`.
    pub fn get_poll_data(&self) -> &InfobusPollData {
        &self.infobus_poll_data
    }

    /// Mirrors `getCreatedAt()`.
    pub fn get_created_at(&self) -> i64 {
        self.created_at
    }

    /// Mirrors `getCreatedAtFormatted()`.
    pub fn get_created_at_formatted(&self) -> String {
        DateUtil::get_date(self.created_at, LONG_DATE)
    }
}
