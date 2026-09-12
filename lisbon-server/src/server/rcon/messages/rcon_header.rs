//! Mirrors `net.h4bbo.lisbon.server.rcon.messages.RconHeader`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RconHeader {
    RefreshLooks,
    HotelAlert,
    RefreshClub,
    RefreshTags,
    RefreshHand,
    RefreshCredits,
    FriendRequest,
    RefreshMessengerCategories,
    RefreshTradeSetting,
    GroupDeleted,
    RefreshGroup,
    RefreshGroupPerms,
    RefreshAds,
    InfobusPoll,
    InfobusDoorStatus,
    InfobusEndEvent,
    RefreshCatalogueFrontpage,
    ClearPhoto,
    DisconnectUser,
    RefreshStatistics,
    RefreshRoomBadges,
}

impl RconHeader {
    /// Mirrors `getRawHeader()`.
    pub fn get_raw_header(&self) -> &'static str {
        match self {
            RconHeader::RefreshLooks => "refresh_looks",
            RconHeader::HotelAlert => "hotel_alert",
            RconHeader::RefreshClub => "refresh_club",
            RconHeader::RefreshTags => "refresh_tags",
            RconHeader::RefreshHand => "refresh_hand",
            RconHeader::RefreshCredits => "refresh_credits",
            RconHeader::FriendRequest => "friendrequest",
            RconHeader::RefreshMessengerCategories => "refreshmessengercategories",
            RconHeader::RefreshTradeSetting => "refreshtrade",
            RconHeader::GroupDeleted => "groupdeleted",
            RconHeader::RefreshGroup => "refreshgroup",
            RconHeader::RefreshGroupPerms => "refreshgroupperms",
            RconHeader::RefreshAds => "refreshads",
            RconHeader::InfobusPoll => "infobuspoll",
            RconHeader::InfobusDoorStatus => "infobusdoorstatus",
            RconHeader::InfobusEndEvent => "infobusendevent",
            RconHeader::RefreshCatalogueFrontpage => "refreshcataloguefrontpage",
            RconHeader::ClearPhoto => "clearphoto",
            RconHeader::DisconnectUser => "disconnect",
            RconHeader::RefreshStatistics => "refreshstats",
            RconHeader::RefreshRoomBadges => "refreshroombadges",
        }
    }

    /// Mirrors `getByHeader(String)`.
    pub fn get_by_header(header: &str) -> Option<RconHeader> {
        for value in Self::all() {
            if value.get_raw_header().eq_ignore_ascii_case(header) {
                return Some(*value);
            }
        }

        None
    }

    fn all() -> &'static [RconHeader] {
        &[
            RconHeader::RefreshLooks,
            RconHeader::HotelAlert,
            RconHeader::RefreshClub,
            RconHeader::RefreshTags,
            RconHeader::RefreshHand,
            RconHeader::RefreshCredits,
            RconHeader::FriendRequest,
            RconHeader::RefreshMessengerCategories,
            RconHeader::RefreshTradeSetting,
            RconHeader::GroupDeleted,
            RconHeader::RefreshGroup,
            RconHeader::RefreshGroupPerms,
            RconHeader::RefreshAds,
            RconHeader::InfobusPoll,
            RconHeader::InfobusDoorStatus,
            RconHeader::InfobusEndEvent,
            RconHeader::RefreshCatalogueFrontpage,
            RconHeader::ClearPhoto,
            RconHeader::DisconnectUser,
            RconHeader::RefreshStatistics,
            RconHeader::RefreshRoomBadges,
        ]
    }
}
