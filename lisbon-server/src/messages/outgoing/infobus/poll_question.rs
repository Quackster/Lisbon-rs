//! Mirrors `net.h4bbo.lisbon.messages.outgoing.infobus.POLL_QUESTION`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct POLL_QUESTION {
    question: String,
    answers: Vec<String>,
}

impl POLL_QUESTION {
    /// Mirrors the `POLL_QUESTION(String, List<String>)` constructor.
    pub fn new(question: &str, answers: Vec<String>) -> Self {
        Self {
            question: question.to_string(),
            answers,
        }
    }
}

impl MessageComposer for POLL_QUESTION {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.question.as_str());
        response.write_int(self.answers.len() as i32);

        for (index, answer) in self.answers.iter().enumerate() {
            response.write_int(index as i32);
            response.write_string(answer.as_str());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        79
    }
}
