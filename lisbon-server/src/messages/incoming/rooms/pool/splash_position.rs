//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.pool.SPLASH_POSITION`.
use crate::game::entity::entity::Entity;
use crate::game::game_scheduler::GameScheduler;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::outgoing::rooms::items::show_program::SHOWPROGRAM;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct SPLASH_POSITION;

impl MessageEvent for SPLASH_POSITION {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if !room_user.is_diving() {
            return Ok(());
        }

        if room.get_model().map(|model| model.get_name() != "pool_b").unwrap_or(true) {
            return Ok(());
        }

        let Some(mut current_item) = room_user.get_current_item() else {
            return Ok(());
        };

        if current_item.get_definition().get_sprite() != "poolLift" {
            return Ok(());
        }

        let destination = Position::new(23, 19, 0.0);
        let contents = format!("{},{}", destination.get_x(), destination.get_y());

        room_user.set_status(StatusType::Swim, "");
        room_user.warp(&destination, true, false);

        room.send(&SHOWPROGRAM::new(vec![
            "BIGSPLASH".to_string(),
            "POSITION".to_string(),
            contents,
        ]));

        room_user.set_diving(false);
        room_user.walk_to(20, 19);
        room_user.set_enable_walking_on_stop(true);

        current_item.show_program(Some("open"));

        let player_id = player.get_details().get_id();
        let player_name = player.get_details().get_name().to_string();
        let instance_id = room_user.get_instance_id();

        GameScheduler::get_instance().schedule(
            move || {
                let mut total: i32 = 0;
                let mut sum: i32 = 0;
                let mut final_score = 0.0;

                for p in room.get_entity_manager().get_players() {
                    let p_locked = p.lock();

                    if p_locked.get_details().get_id() == player_id {
                        continue;
                    }

                    if let Some(p_room_user) = p_locked.get_room_user() {
                        if p_room_user.get_lido_vote() > 0 {
                            sum += p_room_user.get_lido_vote();
                            total += 1;
                        }
                    }
                }

                room.send(&SHOWPROGRAM::new(vec![
                    "cam1".to_string(),
                    "targetcamera".to_string(),
                    instance_id.to_string(),
                ]));

                if total > 0 {
                    final_score = StringUtil::format(sum as f64 / total as f64);
                }

                room.send(&SHOWPROGRAM::new(vec![
                    "cam1".to_string(),
                    "showtext".to_string(),
                    format!("{}'s\n score: {}", player_name, final_score),
                ]));

                for p in room.get_entity_manager().get_players() {
                    let p_locked = p.lock();

                    if let Some(p_room_user) = p_locked.get_room_user() {
                        p_room_user.set_lido_vote(0);
                    }
                }
            },
            1000,
        );

        Ok(())
    }
}
