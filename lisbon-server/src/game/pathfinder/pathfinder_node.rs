//! Mirrors `net.h4bbo.lisbon.game.pathfinder.PathfinderNode`.
//!
//! The Java `Integer cost` sentinel (`Integer.MAX_VALUE`) is an `i32`; the
//! `Comparable<PathfinderNode>` is expressed as [`PathfinderNode::compare`]
//! (the `cost` ordering is intentionally independent of `equals`, which the
//! Rust `Ord` contract forbids).

use std::cmp::Ordering;

use crate::game::pathfinder::position::Position;

#[derive(Debug)]
pub struct PathfinderNode {
    position: Position,
    next_node: Option<Box<PathfinderNode>>,
    cost: i32,
    in_open: bool,
    in_closed: bool,
}

impl PathfinderNode {
    /// Mirrors the `PathfinderNode(Position)` constructor.
    pub fn new(current: Position) -> Self {
        Self {
            position: current,
            next_node: None,
            cost: i32::MAX,
            in_open: false,
            in_closed: false,
        }
    }

    /// Mirrors `getPosition()`.
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Mirrors `setPosition(Position)`.
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Mirrors `getNextNode()`.
    pub fn get_next_node(&self) -> Option<&PathfinderNode> {
        self.next_node.as_deref()
    }

    /// Mirrors `setNextNode(PathfinderNode)`.
    pub fn set_next_node(&mut self, next_node: Option<Box<PathfinderNode>>) {
        self.next_node = next_node;
    }

    /// Mirrors `getCost()`.
    pub fn get_cost(&self) -> i32 {
        self.cost
    }

    /// Mirrors `setCost(int)`.
    pub fn set_cost(&mut self, cost: i32) {
        self.cost = cost;
    }

    /// Mirrors `isInOpen()`.
    pub fn is_in_open(&self) -> bool {
        self.in_open
    }

    /// Mirrors `setInOpen(boolean)`.
    pub fn set_in_open(&mut self, in_open: bool) {
        self.in_open = in_open;
    }

    /// Mirrors `isInClosed()`.
    pub fn is_in_closed(&self) -> bool {
        self.in_closed
    }

    /// Mirrors `setInClosed(boolean)`.
    pub fn set_in_closed(&mut self, in_closed: bool) {
        self.in_closed = in_closed;
    }

    /// Mirrors `compareTo(PathfinderNode)`.
    pub fn compare(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }

    /// Mirrors the `equals(PathfinderNode)` overload (the `Position`
    /// `PartialEq` only compares X and Y — quirk preserved).
    pub fn equals_node(&self, node: &Self) -> bool {
        node.get_position() == self.get_position()
    }
}

impl PartialEq for PathfinderNode {
    /// Mirrors the Java `equals` override (position comparison only).
    fn eq(&self, other: &Self) -> bool {
        self.get_position() == other.get_position()
    }
}

impl Eq for PathfinderNode {}

// Port note: the Java `hashCode` (position hash) is not mirrored; the Rust
// `Position` has no `Hash` impl (its `f64` `z` is not hashable) and its
// `equals`/`hashCode` pair would be inconsistent with that.
