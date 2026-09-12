//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.OBJECTS_WORLD`.
use rand::Rng;

use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct OBJECTS_WORLD {
    items: Option<Vec<Item>>,
    data: Option<Vec<String>>,
}

impl OBJECTS_WORLD {
    /// Mirrors the `OBJECTS_WORLD(List<Item>)` constructor.
    pub fn new(items: Vec<Item>) -> Self {
        Self {
            items: Some(items),
            data: None,
        }
    }

    /// Mirrors the `OBJECTS_WORLD(String)` constructor.
    pub fn from_data(old: &str) -> Self {
        Self {
            items: None,
            data: Some(old.split('\r').map(|line| line.to_string()).collect()),
        }
    }
}

impl MessageComposer for OBJECTS_WORLD {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if let Some(items) = &self.items {
            response.write_int(items.len() as i32);

            for item in items {
                item.serialise(response);
            }
        } else if let Some(data) = &self.data {
            response.write_int(data.len() as i32);

            for entry in data {
                let parts: Vec<&str> = entry.split(' ').collect();

                // `UUID.randomUUID().toString().split("-")[0]`: 8 hex chars.
                let mut uuid = String::new();
                let mut rng = rand::thread_rng();
                for _ in 0..8 {
                    uuid.push(format!("{:x}", rng.gen_range(0..16))
                        .chars()
                        .next()
                        .unwrap());
                }

                response.write_delimeter(uuid, ' ');
                response.write_string(parts.get(1).copied().unwrap_or(""));
                response.write_delimeter(
                    parts.get(2).and_then(|part| part.parse::<i32>().ok()).unwrap_or(0),
                    ' ',
                );
                response.write_delimeter(
                    parts.get(3).and_then(|part| part.parse::<i32>().ok()).unwrap_or(0),
                    ' ',
                );
                response.write_delimeter(
                    parts.get(4).and_then(|part| part.parse::<i32>().ok()).unwrap_or(0),
                    ' ',
                );
                response.write(parts.get(5).and_then(|part| part.parse::<i32>().ok()).unwrap_or(0));
                response.write('\u{000D}');
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        30 // "@^"
    }
}
