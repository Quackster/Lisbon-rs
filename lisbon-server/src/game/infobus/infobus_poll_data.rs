//! Mirrors `net.h4bbo.lisbon.game.infobus.InfobusPollData`.

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InfobusPollData {
    #[serde(default)]
    question: String,
    #[serde(default)]
    answers: Vec<String>,
}

impl InfobusPollData {
    /// Mirrors the `InfobusPollData(String)` constructor.
    pub fn new(question: &str) -> Self {
        Self {
            question: question.to_string(),
            answers: Vec::new(),
        }
    }

    /// Mirrors `getQuestion()`.
    pub fn get_question(&self) -> &str {
        &self.question
    }

    /// Mirrors `getAnswers()`.
    pub fn get_answers(&self) -> &Vec<String> {
        &self.answers
    }

    /// Mirrors `getAnswers().addAll(...)`.
    pub fn add_answers(&mut self, answers: Vec<String>) {
        self.answers.extend(answers);
    }
}
