//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.DefaultInteractor`.
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::GenericTrigger;

pub struct DefaultInteractor {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl DefaultInteractor {
    /// Mirrors the `DefaultInteractor()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onInteract(Player, Room, Item, int)`.
    pub fn on_interact(
        &self,
        _player: &Player,
        _room: &Room,
        _item: &Item,
        _status: i32,
    ) {}
}

impl Default for DefaultInteractor {
    fn default() -> Self {
        Self::new()
    }
}
