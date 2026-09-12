//! Mirrors `org.alexdev.http.game.friends.FriendsFeed`.

use lisbon_server::dao::mysql::alerts_dao::AlertsDao;

use crate::duckhttpd::Template;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::web_connection::WebConnection;

pub struct FriendsFeed;

impl FriendsFeed {
    /// Mirrors `createFriendsOnline(WebConnection, Template)`.
    /// Port note: the Java `(PlayerDetails) template.get("playerDetails")` cast
    // is not expressible: `TemplateValue` materialises context values as JSON
    // and the ported `PlayerDetails` is not `Serialize`/`Deserialize`, so the
    // player id degrades to `0`; the online-friend list (`MessengerUser` is
    // not `Serialize`) degrades to JSON null.
    pub fn create_friends_online(web_connection: &WebConnection, template: &mut dyn Template) {
        if !web_connection.session().get_boolean("authenticated") {
            return;
        }

        if template.get("playerDetails").is_none() {
            return;
        }

        let _friends = AlertsDao::get_online_friends(0);
        let requests = AlertsDao::count_requests(0);

        template.set("feedFriendsOnline", TemplateValue::json(serde_json::Value::Null));
        template.set("feedFriendRequests", TemplateValue::json(serde_json::json!(requests)));

        web_connection.session().delete("friendsOnlineRequest");
    }
}
