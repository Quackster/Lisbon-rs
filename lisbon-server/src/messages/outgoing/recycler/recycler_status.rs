//! Mirrors `net.h4bbo.lisbon.messages.outgoing.recycler.RECYCLER_STATUS`.
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::recycler::recycler_session::RecyclerSession;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct RECYCLER_STATUS {
    recycler_enabled: bool,
    session: Option<RecyclerSession>,
}

impl RECYCLER_STATUS {
    /// Mirrors the `RECYCLER_STATUS(boolean, RecyclerSession)` constructor.
    pub fn new(recycler_enabled: bool, session: Option<RecyclerSession>) -> Self {
        Self {
            recycler_enabled,
            session,
        }
    }
}

impl MessageComposer for RECYCLER_STATUS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if !self.recycler_enabled || self.session.is_none() {
            response.write_int(0);
            return;
        }

        let session = self.session.as_ref().unwrap();

        if session.has_timeout() {
            response.write_int(3);
        } else {
            response.write_int(if session.is_recycling_done() {
                2
            } else {
                1
            });

            let definition = session
                .get_recycler_reward()
                .and_then(|reward| reward.get_catalogue_item())
                .and_then(|item| item.get_definition());

            match definition {
                Some(definition) => {
                    response.write_int(
                        definition
                            .has_behaviour(ItemBehaviour::WallItem) as i32,
                    );
                    response.write_string(definition.get_sprite());

                    if !session.is_recycling_done() {
                        let minutes_left = session.get_minutes_left();
                        response.write_int(if minutes_left % 60 == 0 {
                            minutes_left - 1
                        } else {
                            minutes_left
                        });
                    }
                }
                // The Java NPEs for a missing reward / catalogue item /
                // definition.
                None => {}
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        304 // "Dp"
    }
}
