//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.util.SnowStormEvent`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SnowStormEvent {
    Walk,
    CreateSnowball,
    ThrowSnowballAtLocation,
    ThrowSnowballAtPerson,
}

impl SnowStormEvent {
    /// Mirrors `getEvent(int)`.
    pub fn get_event(event_id: i32) -> Option<SnowStormEvent> {
        for event in [
            SnowStormEvent::Walk,
            SnowStormEvent::CreateSnowball,
            SnowStormEvent::ThrowSnowballAtLocation,
            SnowStormEvent::ThrowSnowballAtPerson,
        ] {
            if event.get_event_id() == event_id {
                return Some(event);
            }
        }

        None
    }

    /// Mirrors `getEventId()`.
    pub fn get_event_id(&self) -> i32 {
        match self {
            Self::Walk => 0,
            Self::CreateSnowball => 3,
            Self::ThrowSnowballAtLocation => 2,
            Self::ThrowSnowballAtPerson => 1,
        }
    }
}
