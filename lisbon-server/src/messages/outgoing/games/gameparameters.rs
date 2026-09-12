//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.GAMEPARAMETERS`.
use crate::game::games::game_parameter::GameParameter;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct GAMEPARAMETERS {
    parameters: Vec<GameParameter>,
}

impl GAMEPARAMETERS {
    /// Mirrors the `GAMEPARAMETERS(GameParameter[])` constructor.
    pub fn new(parameters: Vec<GameParameter>) -> Self {
        Self { parameters }
    }
}

impl MessageComposer for GAMEPARAMETERS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.parameters.len() as i32);

        for parameter in &self.parameters {
            response.write_string(parameter.get_name());
            response.write_bool(!parameter.has_min_max());
            response.write_int(if parameter.is_editable() { 2 } else { 0 });

            if parameter.has_min_max() {
                response.write_int(
                    parameter
                        .get_default_value()
                        .parse::<i32>()
                        .unwrap_or(0),
                );

                if parameter.get_min() != -1 {
                    response.write_bool(true);
                    response.write_int(parameter.get_min());
                }

                if parameter.get_max() != -1 {
                    response.write_bool(true);
                    response.write_int(parameter.get_max());
                }
            } else {
                response.write_string(parameter.get_default_value());
                response.write_int(0);
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        235 // "Ck"
    }
}
