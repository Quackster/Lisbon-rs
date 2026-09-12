//! Mirrors `net.h4bbo.lisbon.game.fuserights.Fuseright`.

use crate::game::player::player_rank::PlayerRank;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fuseright {
    Default,
    Login,
    Trade,
    BuyCredits,
    BuyCreditsFuseLogin,
    RoomQueueDefault,
    Mute,
    Kick,
    ReceiveCallsForHelp,
    RemovePhotos,
    RemoveStickies,
    Mod,
    ModeratorAccess,
    ChatLog,
    RoomAlert,
    RoomKick,
    IgnoreRoomOwner,
    EnterFullRooms,
    EnterLockedRooms,
    SeeAllRoomowners,
    SearchUsers,
    Ban,
    SeeChatLogLink,
    CancelRooMEvent,
    AdministratorAccess,
    AnyRoomController,
    PickUpAnyFurni,
    SeeFlatIds,
    Credits,
    PriorityAccess,
    UseSpecialRoomLayouts,
    UseClubOutfits,
    UseClubOutfitsDefault,
    UseClubBadge,
    UseClubDance,
    UserListCommand,
    FurniListCommand,
    ExtendedBuddylist,
    RoomQueueClub,
    HousekeepingIntra,
    HousekeepingKick,
    HousekeepingAlert,
    HousekeepingDiscussion,
    HousekeepingDiscussionAdmin,
    HousekeepingAdmin,
    HousekeepingAdminCredits,
    HousekeepingAdminUserData,
    HousekeepingAdminPayments,
    HousekeepingCampaign,
    HousekeepingCampaignAds,
    HousekeepingGeorge,
    HousekeepingAdminCatalog,
    HousekeepingHobba,
    HousekeepingHobbaNewbieTools,
    HousekeepingHobbaModeratorTools,
    HousekeepingHobbaHobbaTools,
    HousekeepingHobbaAdminTools,
    HousekeepingHobbaSuperTools,
    HousekeepingBan,
    HousekeepingMegaban,
    HousekeepingSuperban,
}

impl Fuseright {
    /// Const array mirroring `values()`.
    pub const ALL: [Fuseright; 61] = [
        Fuseright::Default,
        Fuseright::Login,
        Fuseright::Trade,
        Fuseright::BuyCredits,
        Fuseright::BuyCreditsFuseLogin,
        Fuseright::RoomQueueDefault,
        Fuseright::Mute,
        Fuseright::Kick,
        Fuseright::ReceiveCallsForHelp,
        Fuseright::RemovePhotos,
        Fuseright::RemoveStickies,
        Fuseright::Mod,
        Fuseright::ModeratorAccess,
        Fuseright::ChatLog,
        Fuseright::RoomAlert,
        Fuseright::RoomKick,
        Fuseright::IgnoreRoomOwner,
        Fuseright::EnterFullRooms,
        Fuseright::EnterLockedRooms,
        Fuseright::SeeAllRoomowners,
        Fuseright::SearchUsers,
        Fuseright::Ban,
        Fuseright::SeeChatLogLink,
        Fuseright::CancelRooMEvent,
        Fuseright::AdministratorAccess,
        Fuseright::AnyRoomController,
        Fuseright::PickUpAnyFurni,
        Fuseright::SeeFlatIds,
        Fuseright::Credits,
        Fuseright::PriorityAccess,
        Fuseright::UseSpecialRoomLayouts,
        Fuseright::UseClubOutfits,
        Fuseright::UseClubOutfitsDefault,
        Fuseright::UseClubBadge,
        Fuseright::UseClubDance,
        Fuseright::UserListCommand,
        Fuseright::FurniListCommand,
        Fuseright::ExtendedBuddylist,
        Fuseright::RoomQueueClub,
        Fuseright::HousekeepingIntra,
        Fuseright::HousekeepingKick,
        Fuseright::HousekeepingAlert,
        Fuseright::HousekeepingDiscussion,
        Fuseright::HousekeepingDiscussionAdmin,
        Fuseright::HousekeepingAdmin,
        Fuseright::HousekeepingAdminCredits,
        Fuseright::HousekeepingAdminUserData,
        Fuseright::HousekeepingAdminPayments,
        Fuseright::HousekeepingCampaign,
        Fuseright::HousekeepingCampaignAds,
        Fuseright::HousekeepingGeorge,
        Fuseright::HousekeepingAdminCatalog,
        Fuseright::HousekeepingHobba,
        Fuseright::HousekeepingHobbaNewbieTools,
        Fuseright::HousekeepingHobbaModeratorTools,
        Fuseright::HousekeepingHobbaHobbaTools,
        Fuseright::HousekeepingHobbaAdminTools,
        Fuseright::HousekeepingHobbaSuperTools,
        Fuseright::HousekeepingBan,
        Fuseright::HousekeepingMegaban,
        Fuseright::HousekeepingSuperban,
    ];

    /// Mirrors `isClubOnly`.
    pub fn is_club_only(&self) -> bool {
        matches!(
            self,
            Self::PriorityAccess
                | Self::UseSpecialRoomLayouts
                | Self::UseClubOutfits
                | Self::UseClubOutfitsDefault
                | Self::UseClubBadge
                | Self::UseClubDance
                | Self::UserListCommand
                | Self::FurniListCommand
                | Self::ExtendedBuddylist
                | Self::RoomQueueClub
        )
    }

    /// Mirrors `getMinimumRank`.
    pub fn minimum_rank(&self) -> Option<PlayerRank> {
        match self {
            Self::Default
            | Self::Login
            | Self::Trade
            | Self::BuyCredits
            | Self::BuyCreditsFuseLogin
            | Self::RoomQueueDefault => Some(PlayerRank::Normal),
            Self::Mute | Self::Kick | Self::ReceiveCallsForHelp | Self::HousekeepingIntra => {
                Some(PlayerRank::Hobba)
            }
            Self::RemovePhotos
            | Self::RemoveStickies
            | Self::HousekeepingAlert
            | Self::HousekeepingDiscussionAdmin
            | Self::HousekeepingHobbaSuperTools => Some(PlayerRank::SuperHobba),
            Self::Mod
            | Self::ModeratorAccess
            | Self::ChatLog
            | Self::RoomAlert
            | Self::RoomKick
            | Self::IgnoreRoomOwner
            | Self::EnterFullRooms
            | Self::EnterLockedRooms
            | Self::SeeAllRoomowners
            | Self::SearchUsers
            | Self::Ban
            | Self::SeeChatLogLink
            | Self::CancelRooMEvent
            | Self::HousekeepingGeorge
            | Self::HousekeepingBan => Some(PlayerRank::Moderator),
            Self::AdministratorAccess
            | Self::AnyRoomController
            | Self::PickUpAnyFurni
            | Self::SeeFlatIds
            | Self::Credits
            | Self::HousekeepingKick
            | Self::HousekeepingAdmin
            | Self::HousekeepingAdminCredits
            | Self::HousekeepingAdminUserData
            | Self::HousekeepingAdminPayments
            | Self::HousekeepingAdminCatalog
            | Self::HousekeepingHobbaAdminTools
            | Self::HousekeepingMegaban
            | Self::HousekeepingSuperban => Some(PlayerRank::Administrator),
            Self::HousekeepingCampaign | Self::HousekeepingCampaignAds => {
                Some(PlayerRank::CommunityManager)
            }
            Self::HousekeepingDiscussion | Self::HousekeepingHobba | Self::HousekeepingHobbaNewbieTools => {
                Some(PlayerRank::Guide)
            }
            Self::HousekeepingHobbaModeratorTools | Self::HousekeepingHobbaHobbaTools => {
                Some(PlayerRank::Hobba)
            }
            Self::PriorityAccess
            | Self::UseSpecialRoomLayouts
            | Self::UseClubOutfits
            | Self::UseClubOutfitsDefault
            | Self::UseClubBadge
            | Self::UseClubDance
            | Self::UserListCommand
            | Self::FurniListCommand
            | Self::ExtendedBuddylist
            | Self::RoomQueueClub => None,
        }
    }
    /// Mirrors `getFuseright`.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Login => "fuse_login",
            Self::Trade => "fuse_trade",
            Self::BuyCredits => "fuse_buy_credits",
            Self::BuyCreditsFuseLogin => "fuse_buy_credits_fuse_login",
            Self::RoomQueueDefault => "fuse_room_queue_default",
            Self::Mute => "fuse_mute",
            Self::Kick => "fuse_kick",
            Self::ReceiveCallsForHelp => "fuse_receive_calls_for_help",
            Self::RemovePhotos => "fuse_remove_photos",
            Self::RemoveStickies => "fuse_remove_stickies",
            Self::Mod => "fuse_mod",
            Self::ModeratorAccess => "fuse_moderator_access",
            Self::ChatLog => "fuse_chat_log",
            Self::RoomAlert => "fuse_room_alert",
            Self::RoomKick => "fuse_room_kick",
            Self::IgnoreRoomOwner => "fuse_ignore_room_owner",
            Self::EnterFullRooms => "fuse_enter_full_rooms",
            Self::EnterLockedRooms => "fuse_enter_locked_rooms",
            Self::SeeAllRoomowners => "fuse_see_all_roomowners",
            Self::SearchUsers => "fuse_search_users",
            Self::Ban => "fuse_ban",
            Self::SeeChatLogLink => "fuse_see_chat_log_link",
            Self::CancelRooMEvent => "fuse_cancel_roomevent",
            Self::AdministratorAccess => "fuse_administrator_access",
            Self::AnyRoomController => "fuse_any_room_controller",
            Self::PickUpAnyFurni => "fuse_pick_up_any_furni",
            Self::SeeFlatIds => "fuse_see_flat_ids",
            Self::Credits => "fuse_credits",
            Self::PriorityAccess => "fuse_priority_access",
            Self::UseSpecialRoomLayouts => "fuse_use_special_room_layouts",
            Self::UseClubOutfits => "fuse_club_outfits",
            Self::UseClubOutfitsDefault => "fuse_club_outfits_default",
            Self::UseClubBadge => "fuse_club_badge",
            Self::UseClubDance => "fuse_club_dance",
            Self::UserListCommand => "fuse_habbo_chooser",
            Self::FurniListCommand => "fuse_furni_chooser",
            Self::ExtendedBuddylist => "fuse_extended_buddylist",
            Self::RoomQueueClub => "fuse_room_queue_club",
            Self::HousekeepingIntra => "housekeeping_intra",
            Self::HousekeepingKick => "housekeeping_kick",
            Self::HousekeepingAlert => "fuse_housekeeping_alert",
            Self::HousekeepingDiscussion => "housekeeping_discussion",
            Self::HousekeepingDiscussionAdmin => "housekeeping_discussion_admin",
            Self::HousekeepingAdmin => "housekeeping_admin",
            Self::HousekeepingAdminCredits => "housekeeping_admin_credits",
            Self::HousekeepingAdminUserData => "housekeeping_admin_user_data",
            Self::HousekeepingAdminPayments => "housekeeping_admin_payments",
            Self::HousekeepingCampaign => "housekeeping_campaign",
            Self::HousekeepingCampaignAds => "housekeeping_campaign_ads",
            Self::HousekeepingGeorge => "housekeeping_george",
            Self::HousekeepingAdminCatalog => "housekeeping_admin_catalog",
            Self::HousekeepingHobba => "housekeeping_hobba",
            Self::HousekeepingHobbaNewbieTools => "housekeeping_hobba_newbietools",
            Self::HousekeepingHobbaModeratorTools => "housekeeping_hobba_moderatortools",
            Self::HousekeepingHobbaHobbaTools => "housekeeping_hobba_hobbatools",
            Self::HousekeepingHobbaAdminTools => "housekeeping_hobba_admintools",
            Self::HousekeepingHobbaSuperTools => "housekeeping_hobba_supertools",
            Self::HousekeepingBan => "housekeeping_ban",
            Self::HousekeepingMegaban => "housekeeping_megaban",
            Self::HousekeepingSuperban => "housekeeping_superban",
        }
    }
}
