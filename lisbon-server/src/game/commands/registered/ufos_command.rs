//! Mirrors `net.h4bbo.lisbon.game.commands.registered.UfosCommand`.

use rand::Rng;

use crate::game::commands::command::Command;
use crate::game::commands::registered::talk_command::TalkCommand;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::messages::outgoing::rooms::items::place_flooritem::PLACE_FLOORITEM;
use crate::messages::outgoing::rooms::items::slide_objectbundle::SLIDEOBJECTBUNDLE;

/// Mirrors `UfosCommand`.
pub struct UfosCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for UfosCommand {
    /// Mirrors the no-arg constructor.
    fn new() -> Self {
        let mut this = Self {
            permissions: Vec::new(),
            arguments: Vec::new(),
        };
        this.add_permissions();
        this.add_arguments();
        this
    }

    /// Mirrors `addPermissions()`.
    fn add_permissions(&mut self) {
        self.permissions.push(Fuseright::AdministratorAccess);
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, _args: &[String]) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let Some(room_user) = player.get_room_user() else {
            return;
        };
        let Some(room) = room_user.get_room() else {
            return;
        };

        if !room.is_owner(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return;
        }

        let mut rng = rand::thread_rng();

        let ufo_amount = rng.gen_range(0..45) + 50;
        let text_to_speech = format!(
            "Help, unknown flying objects! The aliens! There's a swarm of {} ufos coming this way! UFOS! Help! The hotel is attacked! Zap zap zap... Houston, we have a problem! Aliens! soi soi soi soi soi. The aliens are coming! We didn't listen! The end of the world! Aargh! Help, Aliens everywhere! I see ufos! I dream about cheese! I mean, beep beep beep! Meep meep meep! Code Red! Code Red! Area 51! Marihuana! Cape Canaveral! Aaron is a fag! Ufos! {} of them! I see them everywhere! Oh and I see dead people! UFOS! UFOs from Mars! Or from the Moon! Fuck knows! Whatever! Oh my god! They look like fucking weirdos! Space monsters! They look even worse than Rick Astley! UFOs! It's the end of the world! Ufos! Ufos! Ufos!",
            ufo_amount, ufo_amount
        );

        TalkCommand::create_voice_speak_message(&room, &text_to_speech);

        for i in 0..ufo_amount {
            let ufo_id = i32::MAX - (i + 1);

            let mut p_item = Item::new();
            p_item.set_id(ufo_id);
            *p_item.get_position_mut() = Position::new(
                rng.gen_range(0..45),
                rng.gen_range(0..45),
                rng.gen_range(-3..10) as f64,
            );

            p_item.get_definition_mut().set_sprite("nest");
            p_item.get_definition_mut().set_length(1);
            p_item.get_definition_mut().set_width(1);
            room.send(&PLACE_FLOORITEM::new(Box::new(p_item.clone())));

            let position = p_item.get_position();
            let dest_x = rng.gen_range(
                -(20 + position.get_x() * 2)..(20 + position.get_y() * 2),
            );
            let dest_y = rng.gen_range(
                -(20 + position.get_y() * 2)..(20 + position.get_x() * 2),
            );
            let dest_z = rng.gen_range(-9..10) as f64;

            room.send(&SLIDEOBJECTBUNDLE::new(
                position.clone(),
                dest_x,
                dest_y,
                dest_z,
                p_item.get_id(),
            ));
        }
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "UFO's :o".to_string()
    }

    /// Mirrors `getPermissions()`.
    fn get_permissions(&self) -> Vec<Fuseright> {
        self.permissions.clone()
    }

    /// Mirrors `getArguments()`.
    fn get_arguments(&self) -> Vec<String> {
        self.arguments.clone()
    }
}
