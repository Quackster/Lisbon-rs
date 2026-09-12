//! Mirrors `org.alexdev.http.dao.RecommendedDao`.

use lisbon_server::dao::storage::{RowGetters, Storage};
use lisbon_server::game::groups::group::Group;

pub struct RecommendedDao;

impl RecommendedDao {
    /// Mirrors `getRecommendedGroups(boolean)`.
    pub fn get_recommended_groups(staff_pick: bool) -> Vec<Group> {
        let mut group_list = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT groups_details.* FROM cms_recommended INNER JOIN groups_details ON cms_recommended.recommended_id = groups_details.id WHERE type = 'GROUP' AND is_staff_pick = {pick}",
                pick = staff_pick as i32
            ),
        ) {
            group_list.push(Group::new(
                row.i32("id").unwrap_or(0),
                row.str("name").unwrap_or_default().as_str(),
                row.str("description").unwrap_or_default().as_str(),
                row.i32("owner_id").unwrap_or(0),
                row.i32("room_id").unwrap_or(0),
                row.str("badge").unwrap_or_default().as_str(),
                row.bool("recommended").unwrap_or(false),
                row.str("background").unwrap_or_default().as_str(),
                row.i32("views").unwrap_or(0),
                row.i32("topics").unwrap_or(0),
                row.i32("group_type").unwrap_or(0),
                row.i32("forum_type").unwrap_or(0),
                row.i32("forum_premission").unwrap_or(0),
                row.str("alias").unwrap_or_default().as_str(),
                row.i64("created_at").unwrap_or(0),
            ));
        }

        group_list
    }
}
