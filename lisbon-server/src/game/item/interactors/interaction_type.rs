//! Mirrors `net.h4bbo.lisbon.game.item.interactors.InteractionType`.
//!
//! Each variant hands back the interactor `GenericTrigger` singleton
//! (mirroring the Java enum-constant instance, which is created once at
//! enum initialisation); callers downcast to the concrete interactor
//! (the Java `(ConcreteInteractor) getTrigger()` cast).

use std::any::Any;

use crate::game::games::triggers::battle_ships_trigger::BattleShipsTrigger;
use crate::game::games::triggers::chess_trigger::ChessTrigger;
use crate::game::games::triggers::poker_trigger::PokerTrigger;
use crate::game::games::triggers::tic_tac_toe_trigger::TicTacToeTrigger;
use crate::game::item::interactors::types::bed_interactor::BedInteractor;
use crate::game::item::interactors::types::chair_interactor::ChairInteractor;
use crate::game::item::interactors::types::default_interactor::DefaultInteractor;
use crate::game::item::interactors::types::pet_food_interactor::PetFoodInteractor;
use crate::game::item::interactors::types::pet_nest_interactor::PetNestInteractor;
use crate::game::item::interactors::types::pet_toy_interactor::PetToyInteractor;
use crate::game::item::interactors::types::pet_water_bowl_interactor::PetWaterBowlInteractor;
use crate::game::item::interactors::types::pool_booth_interactor::PoolBoothInteractor;
use crate::game::item::interactors::types::pool_ladder_interactor::PoolLadderInteractor;
use crate::game::item::interactors::types::pool_lift_interactor::PoolLiftInteractor;
use crate::game::item::interactors::types::queue_tile_interactor::QueueTileInteractor;
use crate::game::item::interactors::types::wobblesquabble::wobble_squabble_join_queue::WobbleSquabbleJoinQueue;
use crate::game::item::interactors::types::wobblesquabble::wobble_squabble_queue_tile::WobbleSquabbleQueueTile;
use crate::game::item::interactors::types::wobblesquabble::wobble_squabble_tile_start::WobbleSquabbleTileStart;

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize)]
pub enum InteractionType {
    Default,
    Bed,
    Chair,
    Teleport,
    VendingMachine,
    Lert,
    Scoreboard,
    Fortune,
    PetNest,
    PetFood,
    PetWaterBowl,
    PetToy,
    TotemLeg,
    TotemHead,
    TotemPlanet,
    PoolEnter,
    PoolExit,
    PoolBooth,
    PoolLift,
    QueueTile,
    GameTicTacToe,
    GameChess,
    GameBattleships,
    GamePoker,
    WsJoinQueue,
    WsQueueTile,
    WsTileStart,
}

impl InteractionType {
    /// Mirrors `InteractionType.valueOf` (case-insensitive here; Java callers
    /// upper-case the input first).
    pub fn from_str(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "DEFAULT" => Some(Self::Default),
            "BED" => Some(Self::Bed),
            "CHAIR" => Some(Self::Chair),
            "TELEPORT" => Some(Self::Teleport),
            "VENDING_MACHINE" => Some(Self::VendingMachine),
            "LERT" => Some(Self::Lert),
            "SCOREBOARD" => Some(Self::Scoreboard),
            "FORTUNE" => Some(Self::Fortune),
            "PET_NEST" => Some(Self::PetNest),
            "PET_FOOD" => Some(Self::PetFood),
            "PET_WATER_BOWL" => Some(Self::PetWaterBowl),
            "PET_TOY" => Some(Self::PetToy),
            "TOTEM_LEG" => Some(Self::TotemLeg),
            "TOTEM_HEAD" => Some(Self::TotemHead),
            "TOTEM_PLANET" => Some(Self::TotemPlanet),
            "POOL_ENTER" => Some(Self::PoolEnter),
            "POOL_EXIT" => Some(Self::PoolExit),
            "POOL_BOOTH" => Some(Self::PoolBooth),
            "POOL_LIFT" => Some(Self::PoolLift),
            "QUEUE_TILE" => Some(Self::QueueTile),
            "GAME_TIC_TAC_TOE" => Some(Self::GameTicTacToe),
            "GAME_CHESS" => Some(Self::GameChess),
            "GAME_BATTLESHIPS" => Some(Self::GameBattleships),
            "GAME_POKER" => Some(Self::GamePoker),
            "WS_JOIN_QUEUE" => Some(Self::WsJoinQueue),
            "WS_QUEUE_TILE" => Some(Self::WsQueueTile),
            "WS_TILE_START" => Some(Self::WsTileStart),
            _ => None,
        }
    }

    /// Mirrors `getTrigger()` (hands back the variant's interactor
    /// singleton; the game variants hand back the concrete
    /// `GameTrigger` subclass so the game-hall callers can downcast to
    /// it).
    pub fn get_trigger(&self) -> Option<&'static (dyn Any + Send + Sync)> {
        macro_rules! singleton {
            ($t:ty) => {{
                static ONCE: std::sync::OnceLock<$t> = std::sync::OnceLock::new();
                ONCE.get_or_init(<$t>::new)
            }};
        }

        Some(match self {
            Self::Default => singleton!(DefaultInteractor),
            Self::Bed => singleton!(BedInteractor),
            Self::Chair => singleton!(ChairInteractor),
            Self::Teleport => singleton!(DefaultInteractor),
            Self::VendingMachine => singleton!(DefaultInteractor),
            Self::Lert => singleton!(DefaultInteractor),
            Self::Scoreboard => singleton!(DefaultInteractor),
            Self::Fortune => singleton!(DefaultInteractor),
            Self::PetNest => singleton!(PetNestInteractor),
            Self::PetFood => singleton!(PetFoodInteractor),
            Self::PetWaterBowl => singleton!(PetWaterBowlInteractor),
            Self::PetToy => singleton!(PetToyInteractor),
            Self::TotemLeg => singleton!(DefaultInteractor),
            Self::TotemHead => singleton!(DefaultInteractor),
            Self::TotemPlanet => singleton!(DefaultInteractor),
            Self::PoolEnter => singleton!(PoolLadderInteractor),
            Self::PoolExit => singleton!(PoolLadderInteractor),
            Self::PoolBooth => singleton!(PoolBoothInteractor),
            Self::PoolLift => singleton!(PoolLiftInteractor),
            Self::QueueTile => singleton!(QueueTileInteractor),
            Self::GameTicTacToe => singleton!(TicTacToeTrigger),
            Self::GameChess => singleton!(ChessTrigger),
            Self::GameBattleships => singleton!(BattleShipsTrigger),
            Self::GamePoker => singleton!(PokerTrigger),
            Self::WsJoinQueue => singleton!(WobbleSquabbleJoinQueue),
            Self::WsQueueTile => singleton!(WobbleSquabbleQueueTile),
            Self::WsTileStart => singleton!(WobbleSquabbleTileStart),
        })
    }
}
