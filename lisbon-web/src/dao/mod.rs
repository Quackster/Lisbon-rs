//! Mirrors `org.alexdev.http.dao.*`.
//!
//! The web game POJOs (`org.alexdev.http.game.*`) are not ported yet; DAOs
//! that return them use small local structs instead.
pub mod community_dao;
pub mod email_dao;
pub mod friend_management_dao;
pub mod group_discussion_dao;
pub mod group_edit_dao;
pub mod guestbook_dao;
pub mod home_edit_dao;
pub mod homes_dao;
pub mod housekeeping;
pub mod housekeeping_dao;
pub mod minimail_dao;
pub mod news_dao;
pub mod rating_dao;
pub mod recommended_dao;
pub mod register_dao;
pub mod reply_dao;
pub mod session_dao;
pub mod site_dao;
pub mod store_dao;
pub mod verify_dao;
pub mod widget_dao;
