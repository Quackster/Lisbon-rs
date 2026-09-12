//! Mirrors `net.h4bbo.lisbon.messages.outgoing.infobus.VOTE_RESULTS`.
use std::collections::HashMap;

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct VOTE_RESULTS {
    question: String,
    answers: Vec<String>,
    answer_results: HashMap<i32, i32>,
    total_answers: i32,
}

impl VOTE_RESULTS {
    /// Mirrors the `VOTE_RESULTS(String, List<String>, Map<Integer, Integer>, int)`
    /// constructor.
    pub fn new(
        question: &str,
        answers: Vec<String>,
        answer_results: HashMap<i32, i32>,
        total_answers: i32,
    ) -> Self {
        Self {
            question: question.to_string(),
            answers,
            answer_results,
            total_answers,
        }
    }
}

impl MessageComposer for VOTE_RESULTS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.question.as_str());
        response.write_int(self.answers.len() as i32);

        for (index, answer) in self.answers.iter().enumerate() {
            response.write_int(index as i32);
            response.write_string(answer.as_str());
            response.write_int(self.answer_results.get(&(index as i32)).copied().unwrap_or(0));
        }

        response.write_int(self.total_answers);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        80
    }
}
