//! Mirrors `net.h4bbo.lisbon.messages.outgoing.tutorial.TUTORIAL_CONFIGURATION`.
use crate::game::tutorial::tutorial_topic::TutorialTopic;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct TUTORIAL_CONFIGURATION {
    tutorial_id: i32,
    tutorial_name: String,
    topics: Vec<TutorialTopic>,
}

impl TUTORIAL_CONFIGURATION {
    /// Mirrors the `TUTORIAL_CONFIGURATION(int, String, List<TutorialTopic>)` constructor.
    pub fn new(tutorial_id: i32, tutorial_name: &str, topics: Vec<TutorialTopic>) -> Self {
        Self {
            tutorial_id,
            tutorial_name: tutorial_name.to_string(),
            topics,
        }
    }
}

impl MessageComposer for TUTORIAL_CONFIGURATION {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.tutorial_id);
        response.write_string(self.tutorial_name.as_str());
        response.write_int(self.topics.len() as i32);

        for topic in &self.topics {
            response.write_int(topic.get_id());
            response.write_string(topic.get_name());
            response.write_int(topic.get_status());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        327
    }
}
