//! Mirrors `net.h4bbo.lisbon.messages.MessageHandler`.
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::log::Log;
use crate::messages::incoming::catalogue::gcap::GCAP;
use crate::messages::incoming::catalogue::gcix::GCIX;
use crate::messages::incoming::catalogue::get_alias_list::GET_ALIAS_LIST;
use crate::messages::incoming::catalogue::grpc::GRPC;
use crate::messages::incoming::club::get_club::GET_CLUB;
use crate::messages::incoming::club::scr_gift_approval::SCR_GIFT_APPROVAL;
use crate::messages::incoming::club::subscribe_club::SUBSCRIBE_CLUB;
use crate::messages::incoming::events::can_create_roomevent::CAN_CREATE_ROOMEVENT;
use crate::messages::incoming::events::create_roomevent::CREATE_ROOMEVENT;
use crate::messages::incoming::events::edit_roomevent::EDIT_ROOMEVENT;
use crate::messages::incoming::events::get_roomevents_by_type::GET_ROOMEVENTS_BY_TYPE;
use crate::messages::incoming::events::get_roomevent_type_count::GET_ROOMEVENT_TYPE_COUNT;
use crate::messages::incoming::events::quit_roomevent::QUIT_ROOMEVENT;
use crate::messages::incoming::games::gameevent::GAMEEVENT;
use crate::messages::incoming::games::gameparametervalues::GAMEPARAMETERVALUES;
use crate::messages::incoming::games::gamerestart::GAMERESTART;
use crate::messages::incoming::games::getinstancelist::GETINSTANCELIST;
use crate::messages::incoming::games::initiatecreategame::INITIATECREATEGAME;
use crate::messages::incoming::games::initiatejoingame::INITIATEJOINGAME;
use crate::messages::incoming::games::kickplayer::KICKPLAYER;
use crate::messages::incoming::games::leavegame::LEAVEGAME;
use crate::messages::incoming::games::observeinstance::OBSERVEINSTANCE;
use crate::messages::incoming::games::requestfullgamestatus::REQUESTFULLGAMESTATUS;
use crate::messages::incoming::games::startgame::STARTGAME;
use crate::messages::incoming::games::unobserveinstance::UNOBSERVEINSTANCE;
use crate::messages::incoming::games::watchgame::WATCHGAME;
use crate::messages::incoming::handshake::generatekey::GENERATEKEY;
use crate::messages::incoming::handshake::get_session_parameters::GET_SESSION_PARAMETERS;
use crate::messages::incoming::handshake::init_crypto::INIT_CRYPTO;
use crate::messages::incoming::handshake::secretkey::SECRETKEY;
use crate::messages::incoming::handshake::sso::SSO;
use crate::messages::incoming::handshake::try_login::TRY_LOGIN;
use crate::messages::incoming::infobus::changeworld::CHANGEWORLD;
use crate::messages::incoming::infobus::trybus::TRYBUS;
use crate::messages::incoming::infobus::vote::VOTE;
use crate::messages::incoming::inventory::getstrip::GETSTRIP;
use crate::messages::incoming::jukebox::add_jukebox_disc::ADD_JUKEBOX_DISC;
use crate::messages::incoming::jukebox::get_jukebox_disks::GET_JUKEBOX_DISCS;
use crate::messages::incoming::jukebox::get_user_song_disks::GET_USER_SONG_DISCS;
use crate::messages::incoming::jukebox::jukebox_playlist_add::JUKEBOX_PLAYLIST_ADD;
use crate::messages::incoming::jukebox::remove_jukebox_disc::REMOVE_JUKEBOX_DISC;
use crate::messages::incoming::jukebox::reset_jukebox::RESET_JUKEBOX;
use crate::messages::incoming::messenger::finduser::FINDUSER;
use crate::messages::incoming::messenger::follow_friend::FOLLOW_FRIEND;
use crate::messages::incoming::messenger::friendlist_update::FRIENDLIST_UPDATE;
use crate::messages::incoming::messenger::invite_friend::INVITE_FRIEND;
use crate::messages::incoming::messenger::messenger_acceptbuddy::MESSENGER_ACCEPTBUDDY;
use crate::messages::incoming::messenger::messenger_declinebuddy::MESSENGER_DECLINEBUDDY;
use crate::messages::incoming::messenger::messenger_getrequests::MESSENGER_GETREQUESTS;
use crate::messages::incoming::messenger::messenger_init::MESSENGERINIT;
use crate::messages::incoming::messenger::messenger_markread::MESSENGER_MARKREAD;
use crate::messages::incoming::messenger::messenger_removebuddy::MESSENGER_REMOVEBUDDY;
use crate::messages::incoming::messenger::messenger_requestbuddy::MESSENGER_REQUESTBUDDY;
use crate::messages::incoming::messenger::messenger_sendmsg::MESSENGER_SENDMSG;
use crate::messages::incoming::moderation::changecallcategory::CHANGECALLCATEGORY;
use crate::messages::incoming::moderation::delete_cry::DELETE_CRY;
use crate::messages::incoming::moderation::messagetocaller::MESSAGETOCALLER;
use crate::messages::incoming::moderation::moderatoraction::MODERATORACTION;
use crate::messages::incoming::moderation::pick_callforhelp::PICK_CALLFORHELP;
use crate::messages::incoming::moderation::request_cfh::REQUEST_CFH;
use crate::messages::incoming::moderation::submit_cfh::SUBMIT_CFH;
use crate::messages::incoming::navigator::add_favorite_room::ADD_FAVORITE_ROOM;
use crate::messages::incoming::navigator::del_favorite_room::DEL_FAVORITE_ROOM;
use crate::messages::incoming::navigator::getfvrf::GETFVRF;
use crate::messages::incoming::navigator::getspacenodeusers::GETSPACENODEUSERS;
use crate::messages::incoming::navigator::getuserflatcats::GETUSERFLATCATS;
use crate::messages::incoming::navigator::navigate::NAVIGATE;
use crate::messages::incoming::navigator::recommended_rooms::RECOMMENDED_ROOMS;
use crate::messages::incoming::navigator::srchf::SRCHF;
use crate::messages::incoming::navigator::suserf::SUSERF;
use crate::messages::incoming::pets::getpetstat::GETPETSTAT;
use crate::messages::incoming::purse::getusercreditlog::GETUSERCREDITLOG;
use crate::messages::incoming::purse::redeem_voucher::REDEEM_VOUCHER;
use crate::messages::incoming::recycler::confirm_furni_recycling::CONFIRM_FURNI_RECYCLING;
use crate::messages::incoming::recycler::get_furni_recycler_configuration::GET_FURNI_RECYCLER_CONFIGURATION;
use crate::messages::incoming::recycler::get_furni_recycler_status::GET_FURNI_RECYCLER_STATUS;
use crate::messages::incoming::recycler::start_furni_recycling::START_FURNI_RECYCLING;
use crate::messages::incoming::register::age_check::AGE_CHECK;
use crate::messages::incoming::register::approveemail::APPROVEEMAIL;
use crate::messages::incoming::register::approvename::APPROVENAME;
use crate::messages::incoming::register::approve_password::APPROVE_PASSWORD;
use crate::messages::incoming::register::gdate::GDATE;
use crate::messages::incoming::register::register::REGISTER;
use crate::messages::incoming::rooms::badges::setbadge::SETBADGE;
use crate::messages::incoming::rooms::dimmer::msg_roomdimmer_change_state::MSG_ROOMDIMMER_CHANGE_STATE;
use crate::messages::incoming::rooms::dimmer::msg_roomdimmer_get_presets::MSG_ROOMDIMMER_GET_PRESETS;
use crate::messages::incoming::rooms::dimmer::msg_roomdimmer_set_preset::MSG_ROOMDIMMER_SET_PRESET;
use crate::messages::incoming::rooms::flatpropbyitem::FLATPROPBYITEM;
use crate::messages::incoming::rooms::getinterest::GETINTEREST;
use crate::messages::incoming::rooms::getroomad::GETROOMAD;
use crate::messages::incoming::rooms::g_hmap::G_HMAP;
use crate::messages::incoming::rooms::g_items::G_ITEMS;
use crate::messages::incoming::rooms::g_objs::G_OBJS;
use crate::messages::incoming::rooms::g_stat::G_STAT;
use crate::messages::incoming::rooms::g_usrs::G_USRS;
use crate::messages::incoming::rooms::gotoflat::GOTOFLAT;
use crate::messages::incoming::rooms::items::addstripitem::ADDSTRIPITEM;
use crate::messages::incoming::rooms::items::convert_furni_to_credits::CONVERT_FURNI_TO_CREDITS;
use crate::messages::incoming::rooms::items::dice_off::DICE_OFF;
use crate::messages::incoming::rooms::items::g_idata::G_IDATA;
use crate::messages::incoming::rooms::items::movestuff::MOVESTUFF;
use crate::messages::incoming::rooms::items::placestuff::PLACESTUFF;
use crate::messages::incoming::rooms::items::presentopen::PRESENTOPEN;
use crate::messages::incoming::rooms::items::removeitem::REMOVEITEM;
use crate::messages::incoming::rooms::items::setitemdata::SETITEMDATA;
use crate::messages::incoming::rooms::items::setitemstate::SETITEMSTATE;
use crate::messages::incoming::rooms::items::setstuffdata::SETSTUFFDATA;
use crate::messages::incoming::rooms::items::spin_wheel_of_fortune::SPIN_WHEEL_OF_FORTUNE;
use crate::messages::incoming::rooms::items::throw_dice::THROW_DICE;
use crate::messages::incoming::rooms::items::useitem::USEITEM;
use crate::messages::incoming::rooms::moderation::assignrights::ASSIGNRIGHTS;
use crate::messages::incoming::rooms::moderation::kick::KICK;
use crate::messages::incoming::rooms::moderation::letuserin::LETUSERIN;
use crate::messages::incoming::rooms::moderation::removeallrights::REMOVEALLRIGHTS;
use crate::messages::incoming::rooms::moderation::removerights::REMOVERIGHTS;
use crate::messages::incoming::rooms::pool::btcks::BTCKS;
use crate::messages::incoming::rooms::pool::dive::DIVE;
use crate::messages::incoming::rooms::pool::sign::SIGN;
use crate::messages::incoming::rooms::pool::splash_position::SPLASH_POSITION;
use crate::messages::incoming::rooms::pool::swimsuit::SWIMSUIT;
use crate::messages::incoming::rooms::room_directory::ROOM_DIRECTORY;
use crate::messages::incoming::rooms::settings::createflat::CREATEFLAT;
use crate::messages::incoming::rooms::settings::deleteflat::DELETEFLAT;
use crate::messages::incoming::rooms::settings::getflatcat::GETFLATCAT;
use crate::messages::incoming::rooms::settings::getflatinfo::GETFLATINFO;
use crate::messages::incoming::rooms::settings::setflatcat::SETFLATCAT;
use crate::messages::incoming::rooms::settings::setflatinfo::SETFLATINFO;
use crate::messages::incoming::rooms::settings::updateflat::UPDATEFLAT;
use crate::messages::incoming::rooms::teleporter::getdoorflat::GETDOORFLAT;
use crate::messages::incoming::rooms::teleporter::intodoor::INTODOOR;
use crate::messages::incoming::rooms::tryflat::TRYFLAT;
use crate::messages::incoming::rooms::user::carrydrink::CARRYDRINK;
use crate::messages::incoming::rooms::user::carryitem::CARRYITEM;
use crate::messages::incoming::rooms::user::chat::CHAT;
use crate::messages::incoming::rooms::user::dance::DANCE;
use crate::messages::incoming::rooms::user::get_user_tags::GET_USER_TAGS;
use crate::messages::incoming::rooms::user::goaway::GOAWAY;
use crate::messages::incoming::rooms::user::iim::IIM;
use crate::messages::incoming::rooms::user::lookto::LOOKTO;
use crate::messages::incoming::rooms::user::quit::QUIT;
use crate::messages::incoming::rooms::user::rateflat::RATEFLAT;
use crate::messages::incoming::rooms::user::set_sound_setting::SET_SOUND_SETTING;
use crate::messages::incoming::rooms::user::shout::SHOUT;
use crate::messages::incoming::rooms::user::stop::STOP;
use crate::messages::incoming::rooms::user::user_cancel_typing::USER_CANCEL_TYPING;
use crate::messages::incoming::rooms::user::user_start_typing::USER_START_TYPING;
use crate::messages::incoming::rooms::user::walk::WALK;
use crate::messages::incoming::rooms::user::wave::WAVE;
use crate::messages::incoming::rooms::user::whisper::WHISPER;
use crate::messages::incoming::songs::burn_song::BURN_SONG;
use crate::messages::incoming::songs::delete_song::DELETE_SONG;
use crate::messages::incoming::songs::edit_song::EDIT_SONG;
use crate::messages::incoming::songs::eject_sound_package::EJECT_SOUND_PACKAGE;
use crate::messages::incoming::songs::get_play_list::GET_PLAY_LIST;
use crate::messages::incoming::songs::get_song_info::GET_SONG_INFO;
use crate::messages::incoming::songs::get_song_list::GET_SONG_LIST;
use crate::messages::incoming::songs::insert_sound_package::INSERT_SOUND_PACKAGE;
use crate::messages::incoming::songs::new_song::NEW_SONG;
use crate::messages::incoming::songs::save_song::SAVE_SONG;
use crate::messages::incoming::songs::save_song_edit::SAVE_SONG_EDIT;
use crate::messages::incoming::songs::save_song_new::SAVE_SONG_NEW;
use crate::messages::incoming::songs::update_play_list::UPDATE_PLAY_LIST;
use crate::messages::incoming::trade::trade_accept::TRADE_ACCEPT;
use crate::messages::incoming::trade::trade_additem::TRADE_ADDITEM;
use crate::messages::incoming::trade::trade_close::TRADE_CLOSE;
use crate::messages::incoming::trade::trade_open::TRADE_OPEN;
use crate::messages::incoming::trade::trade_unaccept::TRADE_UNACCEPT;
use crate::messages::incoming::tutorial::msg_accept_tutor_invitation::MSG_ACCEPT_TUTOR_INVITATION;
use crate::messages::incoming::tutorial::msg_cancel_tutor_invitations::MSG_CANCEL_TUTOR_INVITATIONS;
use crate::messages::incoming::tutorial::msg_cancel_wait_for_tutor_invitations::MSG_CANCEL_WAIT_FOR_TUTOR_INVITATIONS;
use crate::messages::incoming::tutorial::msg_get_tutors_available::MSG_GET_TUTORS_AVAILABLE;
use crate::messages::incoming::tutorial::msg_invite_tutors::MSG_INVITE_TUTORS;
use crate::messages::incoming::tutorial::msg_reject_tutor_invitation::MSG_REJECT_TUTOR_INVITATION;
use crate::messages::incoming::tutorial::msg_remove_account_help_text::MSG_REMOVE_ACCOUNT_HELP_TEXT;
use crate::messages::incoming::tutorial::msg_wait_for_tutor_invitations::MSG_WAIT_FOR_TUTOR_INVITATIONS;
use crate::messages::incoming::tutorial::reset_tutorial::RESET_TUTORIAL;
use crate::messages::incoming::user::badges::getavailablebadges::GETAVAILABLEBADGES;
use crate::messages::incoming::user::badges::getselectedbadges::GETSELECTEDBADGES;
use crate::messages::incoming::user::get_credits::GET_CREDITS;
use crate::messages::incoming::user::get_ignore_list::GET_IGNORE_LIST;
use crate::messages::incoming::user::get_info::GET_INFO;
use crate::messages::incoming::user::get_possible_achievements::GET_POSSIBLE_ACHIEVEMENTS;
use crate::messages::incoming::user::getavailablesets::GETAVAILABLESETS;
use crate::messages::incoming::user::ignore_user::IGNORE_USER;
use crate::messages::incoming::user::pong::PONG;
use crate::messages::incoming::user::settings::get_account_preferences::GET_ACCOUNT_PREFERENCES;
use crate::messages::incoming::user::settings::get_sound_setting::GET_SOUND_SETTING;
use crate::messages::incoming::user::settings::update_account::UPDATE_ACCOUNT;
use crate::messages::incoming::user::test_latency::TEST_LATENCY;
use crate::messages::incoming::user::unignore_user::UNIGNORE_USER;
use crate::messages::incoming::user::update::UPDATE;
use crate::messages::incoming::welcomingparty::accept_tutor_invitation::ACCEPT_TUTOR_INVITATION;
use crate::messages::incoming::welcomingparty::reject_tutor_invitation::REJECT_TUTOR_INVITATION;
use crate::messages::incoming::wobblesquabble::ptm::PTM;
use crate::messages::outgoing::rooms::groups::group_badges::GROUP_BADGES;
use crate::messages::outgoing::rooms::groups::group_info::GROUP_INFO;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::config::server_configuration::ServerConfiguration;

/// Mirrors `MessageHandler`.
pub struct MessageHandler {
    messages: HashMap<i32, Box<dyn MessageEvent + Send + Sync>>,
}

impl MessageHandler {
    /// Mirrors the `MessageHandler` constructor.
    fn new() -> Self {
        let mut this = Self {
            messages: HashMap::new(),
        };

        this.register_handshake_packets();
        this.register_register_packets();
        this.register_purse_packets();
        this.register_user_packets();
        this.register_club_packets();
        this.register_welcoming_party_packets();
        this.register_navigator_packets();
        this.register_room_packets();
        this.register_room_user_packets();
        this.register_pet_packets();
        this.register_room_badges_packets();
        this.register_room_pool_packets();
        this.register_room_settings_packets();
        this.register_room_item_packets();
        this.register_room_teleporter_packets();
        this.register_room_moderation_packets();
        this.register_infobus_packets();
        this.register_room_event_packets();
        this.register_game_moderation_packets();
        this.register_messenger_packets();
        this.register_catalogue_packets();
        this.register_inventory_packets();
        this.register_trade_packets();
        this.register_song_packets();
        this.register_game_packets();
        this.register_jukebox_packets();
        this.register_ecotron_packets();
        this.register_tutor_packets();

        this.register_event(230, GROUP_BADGES_EVENT);
        this.register_event(231, GROUP_INFO_EVENT);

        this
    }

    /// Mirrors `registerEcotronPackets()`.
    fn register_ecotron_packets(&mut self) {
        self.register_event(222, GET_FURNI_RECYCLER_CONFIGURATION);
        self.register_event(223, GET_FURNI_RECYCLER_STATUS);
        self.register_event(225, START_FURNI_RECYCLING);
        self.register_event(226, CONFIRM_FURNI_RECYCLING);
    }

    /// Mirrors `registerHandshakePackets()`.
    fn register_handshake_packets(&mut self) {
        self.register_event(206, INIT_CRYPTO);
        self.register_event(2002, GENERATEKEY);
        self.register_event(202, GENERATEKEY);
        self.register_event(207, SECRETKEY);
        self.register_event(1817, GET_SESSION_PARAMETERS);
        self.register_event(204, SSO);
        self.register_event(4, TRY_LOGIN);
    }

    /// Mirrors `registerRegisterPackets()`.
    fn register_register_packets(&mut self) {
        self.register_event(49, GDATE);
        self.register_event(46, AGE_CHECK);
        self.register_event(42, APPROVENAME);
        self.register_event(203, APPROVE_PASSWORD);
        self.register_event(197, APPROVEEMAIL);
        self.register_event(43, REGISTER);
    }

    /// Mirrors `registerPursePackets()`.
    fn register_purse_packets(&mut self) {
        self.register_event(8, GET_CREDITS);
        self.register_event(127, GETUSERCREDITLOG);
        self.register_event(129, REDEEM_VOUCHER);
    }

    /// Mirrors `registerUserPackets()`.
    fn register_user_packets(&mut self) {
        self.register_event(7, GET_INFO);
        self.register_event(228, GET_ACCOUNT_PREFERENCES);
        self.register_event(196, PONG);
        self.register_event(44, UPDATE);
        self.register_event(370, GET_POSSIBLE_ACHIEVEMENTS);
        // (Unknown): 149 / BU@M@Flol123@H@J01.01.1991@C@Iqwerty123
        self.register_event(360, GET_IGNORE_LIST);
        self.register_event(319, IGNORE_USER);
        self.register_event(322, UNIGNORE_USER);
        self.register_event(228, GET_SOUND_SETTING);
        self.register_event(9, GETAVAILABLESETS);
        self.register_event(149, UPDATE_ACCOUNT);
        self.register_event(315, TEST_LATENCY);
    }

    /// Mirrors `registerClubPackets()`.
    fn register_club_packets(&mut self) {
        self.register_event(26, GET_CLUB);
        self.register_event(190, SUBSCRIBE_CLUB);
        self.register_event(210, SCR_GIFT_APPROVAL);
    }

    /// Mirrors `registerWelcomingPartyPackets()`.
    fn register_welcoming_party_packets(&mut self) {
        self.register_event(357, ACCEPT_TUTOR_INVITATION);
        self.register_event(358, REJECT_TUTOR_INVITATION);
    }

    /// Mirrors `registerNavigatorPackets()`.
    fn register_navigator_packets(&mut self) {
        self.register_event(150, NAVIGATE);
        self.register_event(16, SUSERF);
        self.register_event(151, GETUSERFLATCATS);
        self.register_event(264, RECOMMENDED_ROOMS);
        self.register_event(17, SRCHF);
        self.register_event(154, GETSPACENODEUSERS);
        self.register_event(18, GETFVRF);
        self.register_event(19, ADD_FAVORITE_ROOM);
        self.register_event(20, DEL_FAVORITE_ROOM);
    }

    /// Mirrors `registerRoomPackets()`.
    fn register_room_packets(&mut self) {
        self.register_event(57, TRYFLAT);
        self.register_event(59, GOTOFLAT);
        self.register_event(182, GETINTEREST);
        self.register_event(2, ROOM_DIRECTORY);
        self.register_event(126, GETROOMAD);
        self.register_event(60, G_HMAP);
        self.register_event(62, G_OBJS);
        self.register_event(61, G_USRS);
        self.register_event(64, G_STAT);
        self.register_event(63, G_ITEMS);
        self.register_event(98, LETUSERIN);
        self.register_event(261, RATEFLAT);
        self.register_event(114, PTM);
    }

    /// Mirrors `registerRoomUserPackets()`.
    fn register_room_user_packets(&mut self) {
        self.register_event(53, QUIT);
        self.register_event(75, WALK);
        self.register_event(115, GOAWAY);
        self.register_event(52, CHAT);
        self.register_event(55, SHOUT);
        self.register_event(56, WHISPER);
        self.register_event(317, USER_START_TYPING);
        self.register_event(318, USER_CANCEL_TYPING);
        self.register_event(79, LOOKTO);
        self.register_event(80, CARRYDRINK);
        self.register_event(87, CARRYITEM);
        self.register_event(94, WAVE);
        self.register_event(93, DANCE);
        self.register_event(88, STOP);
        self.register_event(229, SET_SOUND_SETTING);
        self.register_event(117, IIM);
        self.register_event(263, GET_USER_TAGS);
    }

    /// Mirrors `registerPetPackets()`.
    fn register_pet_packets(&mut self) {
        self.register_event(128, GETPETSTAT);
    }

    /// Mirrors `registerRoomBadgesPackets()`.
    fn register_room_badges_packets(&mut self) {
        self.register_event(157, GETAVAILABLEBADGES);
        self.register_event(158, SETBADGE);
        self.register_event(159, GETSELECTEDBADGES);
    }

    /// Mirrors `registerRoomSettingsPackets()`.
    fn register_room_settings_packets(&mut self) {
        self.register_event(21, GETFLATINFO);
        self.register_event(29, CREATEFLAT);
        self.register_event(25, SETFLATINFO);
        self.register_event(24, UPDATEFLAT);
        self.register_event(153, SETFLATCAT);
        self.register_event(152, GETFLATCAT);
        self.register_event(23, DELETEFLAT);
    }

    /// Mirrors `registerRoomItemPackets()`.
    fn register_room_item_packets(&mut self) {
        self.register_event(90, PLACESTUFF);
        self.register_event(73, MOVESTUFF);
        self.register_event(67, ADDSTRIPITEM);
        self.register_event(83, G_IDATA);
        self.register_event(89, USEITEM);
        self.register_event(84, SETITEMDATA);
        self.register_event(214, SETITEMSTATE);
        self.register_event(85, REMOVEITEM);
        self.register_event(74, SETSTUFFDATA);
        self.register_event(183, CONVERT_FURNI_TO_CREDITS);
        self.register_event(76, THROW_DICE);
        self.register_event(77, DICE_OFF);
        self.register_event(247, SPIN_WHEEL_OF_FORTUNE);
        self.register_event(341, MSG_ROOMDIMMER_GET_PRESETS);
        self.register_event(342, MSG_ROOMDIMMER_SET_PRESET);
        self.register_event(343, MSG_ROOMDIMMER_CHANGE_STATE);
        self.register_event(78, PRESENTOPEN);
        self.register_event(99, REMOVEITEM);
    }

    /// Mirrors `registerRoomTeleporterPackets()`.
    pub fn register_room_teleporter_packets(&mut self) {
        self.register_event(81, INTODOOR);
        self.register_event(28, GETDOORFLAT);
    }

    /// Mirrors `registerRoomPoolPackets()`.
    fn register_room_pool_packets(&mut self) {
        self.register_event(116, SWIMSUIT);
        self.register_event(105, BTCKS);
        self.register_event(106, DIVE);
        self.register_event(107, SPLASH_POSITION);
        self.register_event(104, SIGN);
    }

    /// Mirrors `registerRoomModerationPackets()`.
    fn register_room_moderation_packets(&mut self) {
        self.register_event(95, KICK);
        self.register_event(96, ASSIGNRIGHTS);
        self.register_event(97, REMOVERIGHTS);
        self.register_event(155, REMOVEALLRIGHTS);
    }

    /// Mirrors `registerInfobusPackets()`.
    fn register_infobus_packets(&mut self) {
        self.register_event(111, CHANGEWORLD);
        self.register_event(112, VOTE);
        self.register_event(113, TRYBUS);
    }

    /// Mirrors `registerRoomEventPackets()`.
    fn register_room_event_packets(&mut self) {
        self.register_event(345, CAN_CREATE_ROOMEVENT);
        self.register_event(346, CREATE_ROOMEVENT);
        self.register_event(348, EDIT_ROOMEVENT);
        self.register_event(347, QUIT_ROOMEVENT);
        self.register_event(349, GET_ROOMEVENT_TYPE_COUNT);
        self.register_event(350, GET_ROOMEVENTS_BY_TYPE);
    }

    /// Mirrors `registerGameModerationPackets()`.
    fn register_game_moderation_packets(&mut self) {
        self.register_event(200, MODERATORACTION);
        self.register_event(237, REQUEST_CFH);
        self.register_event(86, SUBMIT_CFH);
        self.register_event(48, PICK_CALLFORHELP);
        self.register_event(199, MESSAGETOCALLER);
        self.register_event(198, CHANGECALLCATEGORY);
        self.register_event(238, DELETE_CRY);
    }

    /// Mirrors `registerTradePackets()`.
    fn register_trade_packets(&mut self) {
        self.register_event(71, TRADE_OPEN);
        self.register_event(72, TRADE_ADDITEM);
        self.register_event(70, TRADE_CLOSE);
        self.register_event(69, TRADE_ACCEPT);
        self.register_event(68, TRADE_UNACCEPT);
    }

    /// Mirrors `registerMessengerPackets()`.
    fn register_messenger_packets(&mut self) {
        self.register_event(12, MESSENGERINIT);
        self.register_event(41, FINDUSER);
        self.register_event(39, MESSENGER_REQUESTBUDDY);
        self.register_event(38, MESSENGER_DECLINEBUDDY);
        self.register_event(37, MESSENGER_ACCEPTBUDDY);
        self.register_event(233, MESSENGER_GETREQUESTS);
        // register_event(191, MESSENGER_GETMESSAGES)
        // register_event(36, MESSENGER_ASSIGNPERSMSG)
        self.register_event(40, MESSENGER_REMOVEBUDDY);
        self.register_event(33, MESSENGER_SENDMSG);
        self.register_event(32, MESSENGER_MARKREAD);
        self.register_event(262, FOLLOW_FRIEND);
        self.register_event(15, FRIENDLIST_UPDATE);
        self.register_event(34, INVITE_FRIEND);
    }

    /// Mirrors `registerCataloguePackets()`.
    fn register_catalogue_packets(&mut self) {
        self.register_event(101, GCIX);
        self.register_event(102, GCAP);
        self.register_event(100, GRPC);
        self.register_event(215, GET_ALIAS_LIST);
    }

    /// Mirrors `registerInventoryPackets()`.
    fn register_inventory_packets(&mut self) {
        self.register_event(65, GETSTRIP);
        self.register_event(66, FLATPROPBYITEM);
    }

    /// Mirrors `registerSongPackets()`.
    fn register_song_packets(&mut self) {
        self.register_event(244, GET_SONG_LIST);
        self.register_event(246, GET_SONG_LIST);
        self.register_event(239, NEW_SONG);
        self.register_event(219, INSERT_SOUND_PACKAGE);
        self.register_event(220, EJECT_SOUND_PACKAGE);
        self.register_event(240, SAVE_SONG_NEW);
        self.register_event(243, UPDATE_PLAY_LIST);
        self.register_event(221, GET_SONG_INFO);
        self.register_event(245, GET_PLAY_LIST);
        self.register_event(248, DELETE_SONG);
        self.register_event(241, EDIT_SONG);
        self.register_event(242, SAVE_SONG_EDIT);
        self.register_event(218, SAVE_SONG);
        self.register_event(254, BURN_SONG);
    }

    /// Mirrors `registerGamePackets()`.
    fn register_game_packets(&mut self) {
        self.register_event(159, GETINSTANCELIST);
        self.register_event(160, OBSERVEINSTANCE);
        self.register_event(161, UNOBSERVEINSTANCE);
        self.register_event(162, INITIATECREATEGAME);
        self.register_event(163, GAMEPARAMETERVALUES);
        self.register_event(165, INITIATEJOINGAME);
        self.register_event(167, LEAVEGAME);
        self.register_event(168, KICKPLAYER);
        self.register_event(169, WATCHGAME);
        self.register_event(170, STARTGAME);
        self.register_event(171, GAMEEVENT);
        self.register_event(172, GAMERESTART);
        self.register_event(173, REQUESTFULLGAMESTATUS);
    }

    /// Mirrors `registerTutorPackets()`.
    fn register_tutor_packets(&mut self) {
        self.register_event(356, MSG_INVITE_TUTORS);
        self.register_event(355, MSG_GET_TUTORS_AVAILABLE);
        self.register_event(362, MSG_WAIT_FOR_TUTOR_INVITATIONS);
        self.register_event(363, MSG_CANCEL_WAIT_FOR_TUTOR_INVITATIONS);
        self.register_event(313, MSG_REMOVE_ACCOUNT_HELP_TEXT);
        self.register_event(357, MSG_ACCEPT_TUTOR_INVITATION);
        self.register_event(358, MSG_REJECT_TUTOR_INVITATION);
        self.register_event(359, MSG_CANCEL_TUTOR_INVITATIONS);
        self.register_event(249, RESET_TUTORIAL);
    }

    /// Mirrors `registerJukeboxPackets()`.
    fn register_jukebox_packets(&mut self) {
        self.register_event(258, GET_JUKEBOX_DISCS);
        self.register_event(259, GET_USER_SONG_DISCS);
        self.register_event(255, ADD_JUKEBOX_DISC);
        self.register_event(256, REMOVE_JUKEBOX_DISC);
        self.register_event(257, JUKEBOX_PLAYLIST_ADD);
        self.register_event(260, RESET_JUKEBOX);
    }

    /// Mirrors `registerEvent(int, MessageEvent)`.
    fn register_event(
        &mut self,
        header: i32,
        event: impl MessageEvent + Send + Sync + 'static,
    ) {
        self.messages.insert(header, Box::new(event));
    }

    /// Mirrors `handleRequest(Player, NettyRequest)`.
    pub fn handle_request(&self, player: &mut Player, message: &mut NettyRequest) {
        if ServerConfiguration::get_boolean("log.received.packets") {
            if self.messages.contains_key(&message.get_header_id()) {
                let name = self
                    .messages
                    .get(&message.get_header_id())
                    .map(|event| event.type_name())
                    .unwrap_or("Unknown");
                tracing::info!(
                    parent: player.get_logger(),
                    "Received ({}): {} / {}",
                    name,
                    message.get_header_id(),
                    message.get_message_body()
                );
            } else {
                tracing::info!(
                    parent: player.get_logger(),
                    "Received ({}): {} / {}",
                    "Unknown",
                    message.get_header_id(),
                    message.get_message_body()
                );
            }
        }

        self.invoke(player, message.get_header_id(), message);
    }

    /// Mirrors `invoke(Player, int, NettyRequest)`.
    fn invoke(&self, player: &mut Player, message_id: i32, message: &mut NettyRequest) {
        if let Some(event) = self.messages.get(&message_id) {
            if let Err(error) = event.handle_mut(player, message) {
                let name = if player.is_logged_in() {
                    player.get_details().get_name().to_string()
                } else {
                    String::new()
                };

                Log::get_error_logger().error(format!(
                    "Error occurred when handling ({}) for user ({}): {}",
                    message.get_header_id(),
                    name,
                    error
                ));
            }
        }

        message.dispose();
    }

    /// Mirrors `getMessages()`.
    #[allow(dead_code)]
    fn get_messages(&self) -> &HashMap<i32, Box<dyn MessageEvent + Send + Sync>> {
        &self.messages
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static MessageHandler {
        static INSTANCE: OnceLock<MessageHandler> = OnceLock::new();

        INSTANCE.get_or_init(Self::new)
    }
}

/// Mirrors the anonymous `230` group badges event.
#[allow(non_camel_case_types)]
struct GROUP_BADGES_EVENT;

impl MessageEvent for GROUP_BADGES_EVENT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let mut group_badges: HashMap<i32, String> = HashMap::new();

        for p in room.get_entity_manager().get_players() {
            let p = p.lock();

            if p.get_details().get_favourite_group_id() > 0 {
                if group_badges
                    .contains_key(&p.get_details().get_favourite_group_id())
                {
                    continue;
                }

                let Some(group) = player.get_joined_group(p.get_details().get_favourite_group_id())
                else {
                    continue;
                };

                group_badges.insert(group.get_id(), group.get_badge());
            }
        }

        player.send(&GROUP_BADGES::new(group_badges));

        Ok(())
    }
}

/// Mirrors the anonymous `231` group info event.
#[allow(non_camel_case_types)]
struct GROUP_INFO_EVENT;

impl MessageEvent for GROUP_INFO_EVENT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let group_id = reader.read_int();

        for p in room.get_entity_manager().get_players() {
            let p = p.lock();

            if let Some(group) = p.get_joined_group(group_id) {
                player.send(&GROUP_INFO::new(group.clone()));
                break;
            }
        }

        Ok(())
    }
}
