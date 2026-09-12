//! Mirrors `net.h4bbo.lisbon.game.tutorial.TutorialTopic`.

#[derive(Clone, Debug, Default)]
pub struct TutorialTopic {
    id: i32,
    name: String,
    status: i32,
}

impl TutorialTopic {
    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `setId(int)`.
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `setName(String)`.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Mirrors `getStatus()`.
    pub fn get_status(&self) -> i32 {
        self.status
    }

    /// Mirrors `setStatus(int)`.
    pub fn set_status(&mut self, status: i32) {
        self.status = status;
    }
}
