//! Mirrors `org.alexdev.http.game.news.NewsView`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NewsView {
    Months,
    Archive,
    Default,
}
