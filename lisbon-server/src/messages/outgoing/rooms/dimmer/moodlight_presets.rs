//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.dimmer.MOODLIGHT_PRESETS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct MOODLIGHT_PRESETS {
    current_preset: i32,
    presets: Vec<String>,
}

impl MOODLIGHT_PRESETS {
    /// Mirrors the `MOODLIGHT_PRESETS(int, List<String>)` constructor.
    pub fn new(current_preset: i32, presets: Vec<String>) -> Self {
        Self {
            current_preset,
            presets,
        }
    }
}

impl MessageComposer for MOODLIGHT_PRESETS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.presets.len() as i32);
        response.write_int(self.current_preset);

        let mut i = 1;
        for preset in &self.presets {
            let preset_data: Vec<&str> = preset.split(',').collect();

            response.write_int(i);
            response.write_int(
                preset_data
                    .get(0)
                    .and_then(|part| part.parse::<i32>().ok())
                    .unwrap_or(0),
            );
            response.write_string(preset_data.get(1).copied().unwrap_or("").replace('#', ""));
            response.write_int(
                preset_data
                    .get(2)
                    .and_then(|part| part.parse::<i32>().ok())
                    .unwrap_or(0),
            );
            i += 1;
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        365 // "Em"
    }
}
