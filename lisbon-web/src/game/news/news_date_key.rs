//! Mirrors `org.alexdev.http.game.news.NewsDateKey`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NewsDateKey {
    Yesterday,
    ThisWeek,
    ThisMonth,
    All,
    Today,
    PastYear,
}
