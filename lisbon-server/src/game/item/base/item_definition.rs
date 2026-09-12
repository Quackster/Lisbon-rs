//! Mirrors `net.h4bbo.lisbon.game.item.base.ItemDefinition`.

use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::interactors::interaction_type::InteractionType;

pub const DEFAULT_TOP_HEIGHT: f64 = 0.001;

#[derive(Clone, Debug, serde::Serialize)]
pub struct ItemDefinition {
    id: i32,
    sprite: String,
    behaviour_data: String,
    top_height: f64,
    length: i32,
    width: i32,
    colour: String,
    behaviour_list: Vec<ItemBehaviour>,
    name: String,
    description: String,
    drink_ids: Vec<i32>,
    is_recyclable: bool,
    interaction_type: Option<InteractionType>,
}

impl ItemDefinition {
    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        Self {
            id: 0,
            sprite: String::new(),
            behaviour_data: String::new(),
            top_height: DEFAULT_TOP_HEIGHT,
            length: 1,
            width: 1,
            colour: String::new(),
            behaviour_list: Vec::new(),
            name: String::new(),
            description: String::new(),
            drink_ids: Vec::new(),
            is_recyclable: false,
            interaction_type: None,
        }
    }

    /// Mirrors the 12-arg `ItemDefinition` constructor.
    pub fn with_data(
        id: i32,
        sprite: &str,
        name: &str,
        description: &str,
        behaviour_data: &str,
        interactor: &str,
        top_height: f64,
        length: i32,
        width: i32,
        colour: &str,
        drink_id_data: &str,
        is_recyclable: bool,
    ) -> Self {
        let interaction_type = InteractionType::from_str(&interactor.to_uppercase());
        let behaviour_list = Self::parse_behaviour(behaviour_data);

        let mut drink_ids = Vec::new();
        for data in drink_id_data.split(',') {
            if let Ok(value) = data.parse::<i32>() {
                drink_ids.push(value);
            }
        }

        let mut top_height = top_height;

        if !behaviour_list.contains(&ItemBehaviour::CanSitOnTop)
            && !behaviour_list.contains(&ItemBehaviour::CanLayOnTop)
            && !behaviour_list.contains(&ItemBehaviour::CanStackOnTop)
        {
            top_height = 0.0;
        }

        if top_height == 0.0 {
            top_height = DEFAULT_TOP_HEIGHT;
        }

        Self {
            id,
            sprite: sprite.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            behaviour_data: behaviour_data.to_string(),
            interaction_type,
            top_height,
            length,
            width,
            colour: colour.to_string(),
            is_recyclable,
            behaviour_list,
            drink_ids,
        }
    }

    /// Mirrors `parseBehaviour`.
    fn parse_behaviour(behaviour_data: &str) -> Vec<ItemBehaviour> {
        let mut behaviour_list = Vec::new();

        if !behaviour_data.is_empty() {
            for behaviour in behaviour_data.split(',') {
                // Java's `valueOf` throws on an unknown name; unknown
                // entries are skipped here.
                if let Some(behaviour) = ItemBehaviour::from_str(&behaviour.to_uppercase()) {
                    behaviour_list.push(behaviour);
                }
            }
        }

        behaviour_list
    }

    /// Mirrors `hasBehaviour`.
    pub fn has_behaviour(&self, behaviour: ItemBehaviour) -> bool {
        self.behaviour_list.contains(&behaviour)
    }

    /// Mirrors `addBehaviour`.
    pub fn add_behaviour(&mut self, behaviour: ItemBehaviour) {
        if self.behaviour_list.contains(&behaviour) {
            return;
        }

        self.behaviour_list.push(behaviour);
    }

    /// Mirrors `removeBehaviour`.
    pub fn remove_behaviour(&mut self, behaviour: ItemBehaviour) {
        self.behaviour_list.retain(|b| *b != behaviour);
    }

    /// Mirrors `getIcon`.
    pub fn get_icon(&self, special_sprite_id: i32) -> String {
        let mut icon = self.sprite.clone();

        if special_sprite_id > 0 {
            icon += &format!(" {special_sprite_id}");
        }

        icon
    }

    /// Mirrors `getId`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getSprite`.
    pub fn get_sprite(&self) -> &str {
        &self.sprite
    }

    /// Mirrors `setSprite`.
    pub fn set_sprite(&mut self, sprite: &str) {
        self.sprite = sprite.to_string();
    }

    /// Mirrors `getTopHeight`.
    pub fn get_top_height(&self) -> f64 {
        self.top_height
    }

    /// Mirrors `getPositiveTopHeight`.
    pub fn get_positive_top_height(&self) -> f64 {
        if self.top_height < 0.0 {
            return DEFAULT_TOP_HEIGHT;
        }

        self.top_height
    }

    /// Mirrors `getLength`.
    pub fn get_length(&self) -> i32 {
        self.length
    }

    /// Mirrors `getWidth`.
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Mirrors `getColour`.
    pub fn get_colour(&self) -> &str {
        &self.colour
    }

    /// Mirrors `setTopHeight`.
    pub fn set_top_height(&mut self, top_height: f64) {
        self.top_height = top_height;
    }

    /// Mirrors `getBehaviourData`.
    pub fn get_behaviour_data(&self) -> &str {
        &self.behaviour_data
    }

    /// Mirrors `setLength`.
    pub fn set_length(&mut self, length: i32) {
        self.length = length;
    }

    /// Mirrors `setWidth`.
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    /// Mirrors `getBehaviourList`.
    pub fn get_behaviour_list(&self) -> &[ItemBehaviour] {
        &self.behaviour_list
    }

    /// Mirrors `getName`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `getDescription`.
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Mirrors `getDrinkIds`.
    pub fn get_drink_ids(&self) -> &[i32] {
        &self.drink_ids
    }

    /// Mirrors `getInteractionType`.
    pub fn get_interaction_type(&self) -> Option<InteractionType> {
        self.interaction_type
    }

    /// Mirrors `setInteractionType`.
    pub fn set_interaction_type(&mut self, interaction_type: Option<InteractionType>) {
        self.interaction_type = interaction_type;
    }

    /// Mirrors `isRecyclable`.
    pub fn is_recyclable(&self) -> bool {
        self.is_recyclable
    }

    /// Mirrors `setRecyclable`.
    pub fn set_recyclable(&mut self, recyclable: bool) {
        self.is_recyclable = recyclable;
    }
}
