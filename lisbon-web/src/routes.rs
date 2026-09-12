//! Mirrors `org.alexdev.http.Routes`.

use crate::controllers::api::advertisement_controller;
use crate::controllers::api::imager_controller;
use crate::controllers::api::verify_controller;
use crate::controllers::base_controller;
use crate::controllers::groups::discussions::discussion_actions_controller;
use crate::controllers::groups::discussions::discussion_controller;
use crate::controllers::groups::discussions::discussion_preview_controller;
use crate::controllers::groups::group_controller;
use crate::controllers::groups::group_discussions_controller;
use crate::controllers::groups::group_favourite_controller;
use crate::controllers::groups::group_habblet_controller;
use crate::controllers::groups::group_member_controller;
use crate::controllers::groups::group_tag_controller;
use crate::controllers::habblet::event_controller;
use crate::controllers::habblet::feed_controller;
use crate::controllers::habblet::habbo_club_habblet;
use crate::controllers::habblet::invite_controller;
use crate::controllers::habblet::name_check_controller;
use crate::controllers::habblet::navigation_component;
use crate::controllers::habblet::proxy_habblet;
use crate::controllers::habblet::room_selection_controller;
use crate::controllers::habblet::update_motto_controller;
use crate::controllers::habblet::voucher_controller;
use crate::controllers::habblet::xml_controller;
use crate::controllers::homes::homes_controller;
use crate::controllers::homes::note_editor_controller;
use crate::controllers::homes::store::store_controller;
use crate::controllers::homes::widget_controller;
use crate::controllers::homes::widgets::badges_controller;
use crate::controllers::homes::widgets::friends_widget_controller;
use crate::controllers::homes::widgets::guestbook_controller;
use crate::controllers::homes::widgets::member_widget_controller;
use crate::controllers::homes::widgets::rate_controller;
use crate::controllers::homes::widgets::trax_controller;
use crate::controllers::housekeeping::housekeeping_ads_controller;
use crate::controllers::housekeeping::housekeeping_bans_controller;
use crate::controllers::housekeeping::housekeeping_catalogue_frontpage_controller;
use crate::controllers::housekeeping::housekeeping_commands_controller;
use crate::controllers::housekeeping::housekeeping_config_controller;
use crate::controllers::housekeeping::housekeeping_controller;
use crate::controllers::housekeeping::housekeeping_infobus_controller;
use crate::controllers::housekeeping::housekeeping_news_controller;
use crate::controllers::housekeeping::housekeeping_room_badges_controller;
use crate::controllers::housekeeping::housekeeping_transactions_controller;
use crate::controllers::housekeeping::housekeeping_users_controller;
use crate::controllers::site::account_controller;
use crate::controllers::site::client_controller;
use crate::controllers::site::club_controller;
use crate::controllers::site::collectables_controller;
use crate::controllers::site::community_controller;
use crate::controllers::site::credits_controller;
use crate::controllers::site::faq_controller;
use crate::controllers::site::friend_management_controller;
use crate::controllers::site::games_controller;
use crate::controllers::site::homepage_controller;
use crate::controllers::site::minimail_controller;
use crate::controllers::site::news_controller;
use crate::controllers::site::profile_controller;
use crate::controllers::site::quickmenu_controller;
use crate::controllers::site::recovery_controller;
use crate::controllers::site::register_controller;
use crate::controllers::site::site_controller;
use crate::controllers::site::tag_controller;
use crate::duckhttpd::RouteManager;
use crate::duckhttpd::RouteHandler;

/// Mirrors `Routes.HOUSEKEEPING_PATH`.
pub const HOUSEKEEPING_PATH: &str = "allseeingeye/hk";

fn add_route(paths: Vec<String>, handler: RouteHandler) {
    let borrowed: Vec<&str> = paths.iter().map(String::as_str).collect();
    RouteManager::add_route(&borrowed, handler);
}

/// Mirrors `Routes.register()`.
pub fn register_routes() {
    RouteManager::add_route(
        &["/", "/index", "/home"],
        homepage_controller::homepage,
    );
    RouteManager::add_route(&["/maintenance"], homepage_controller::maintenance);

    RouteManager::add_route(&[""], base_controller::handle_route);

    // Site
    RouteManager::add_route(&["/me"], account_controller::me);
    RouteManager::add_route(&["/welcome"], account_controller::welcome);

    // News
    RouteManager::add_route(&["/articles"], news_controller::articles);
    RouteManager::add_route(&["/articles/archive"], news_controller::articles);
    RouteManager::add_route(&["/articles/category/*"], news_controller::articles);
    RouteManager::add_route(&["/articles/*-*"], news_controller::articles);

    // Events
    RouteManager::add_route(&["/community/events"], news_controller::events);
    RouteManager::add_route(&["/community/events/archive"], news_controller::events);
    RouteManager::add_route(
        &["/community/events/category/*"],
        news_controller::events,
    );
    RouteManager::add_route(&["/community/events/*-*"], news_controller::events);

    // Fansites
    RouteManager::add_route(&["/community/fansites"], news_controller::fansites);
    RouteManager::add_route(
        &["/community/fansites/archive"],
        news_controller::fansites,
    );
    // Mirrors the Java bug: this route maps to `events`, not `fansites`.
    RouteManager::add_route(
        &["/community/fansites/category/*"],
        news_controller::events,
    );
    RouteManager::add_route(
        &["/community/fansites/*-*"],
        news_controller::fansites,
    );

    // Site
    RouteManager::add_route(&["/community"], community_controller::community);
    RouteManager::add_route(&["/games"], games_controller::games);
    RouteManager::add_route(
        &["/games/score_all_time"],
        games_controller::games_all_time,
    );
    RouteManager::add_route(
        &["/habblet/personalhighscores"],
        games_controller::personalhighscores,
    );
    RouteManager::add_route(&["/credits"], credits_controller::credits);
    RouteManager::add_route(
        &["/credits/history"],
        credits_controller::transactions,
    );
    RouteManager::add_route(&["/credits/pixels"], site_controller::pixels);
    RouteManager::add_route(&["/credits/club"], club_controller::club);
    RouteManager::add_route(
        &["/credits/collectables"],
        collectables_controller::collectables,
    );
    RouteManager::add_route(
        &["/credits/club/tryout"],
        club_controller::club_tryout,
    );
    RouteManager::add_route(&["/tag"], tag_controller::tag);
    RouteManager::add_route(&["/tag/*"], tag_controller::search);
    RouteManager::add_route(&["/help/*"], faq_controller::faq);

    // Client
    RouteManager::add_route(
        &["/components/updateHabboCount"],
        client_controller::update_habbo_count,
    );
    RouteManager::add_route(&["/client"], client_controller::client);
    RouteManager::add_route(&["/clientlog/update"], client_controller::blank);
    RouteManager::add_route(&["/cacheCheck"], client_controller::blank);
    RouteManager::add_route(
        &["/shockwave_client"],
        client_controller::shockwaveclient,
    );
    RouteManager::add_route(
        &["/flash_client"],
        client_controller::flash_client,
    );
    // /beta_client: deprecated (R34 client).
    RouteManager::add_route(
        &["/client_popup/install_shockwave"],
        client_controller::client_install_shockwave,
    );
    RouteManager::add_route(&["/client_error"], client_controller::client_error);
    RouteManager::add_route(
        &["/client_connection_failed"],
        client_controller::client_connection_failed,
    );

    // Account
    RouteManager::add_route(&["/account/banned"], account_controller::banned);
    RouteManager::add_route(&["/account/logout"], account_controller::logout);
    RouteManager::add_route(
        &["/account/login"],
        account_controller::login_popup,
    );
    RouteManager::add_route(
        &["/account/password/forgot"],
        recovery_controller::forgot,
    );
    RouteManager::add_route(
        &["/account/password/recovery"],
        recovery_controller::recovery,
    );
    RouteManager::add_route(
        &["/account/activate"],
        recovery_controller::activate,
    );
    RouteManager::add_route(
        &["/login_popup"],
        account_controller::login_popup,
    );
    RouteManager::add_route(&["/account/submit"], account_controller::submit);
    RouteManager::add_route(
        &["/security_check"],
        account_controller::security_check,
    );
    RouteManager::add_route(
        &["/account/reauthenticate"],
        account_controller::reauthenticate,
    );

    // Profile
    RouteManager::add_route(&["/profile"], profile_controller::profile);
    RouteManager::add_route(
        &["/profile/wardrobeStore"],
        profile_controller::wardrobe_store,
    );
    RouteManager::add_route(
        &["/profile/passwordupdate"],
        profile_controller::passwordupdate,
    );
    RouteManager::add_route(
        &["/profile/emailupdate"],
        profile_controller::emailupdate,
    );
    RouteManager::add_route(
        &["/profile/characterupdate"],
        profile_controller::characterupdate,
    );
    RouteManager::add_route(&["/profile/profile.action"], profile_controller::action);
    RouteManager::add_route(
        &["/profile/profileupdate"],
        profile_controller::profileupdate,
    );
    RouteManager::add_route(&["/club"], profile_controller::club);

    RouteManager::add_route(
        &["/friendmanagement/ajax/editCategory"],
        friend_management_controller::edit_category,
    );
    RouteManager::add_route(
        &["/friendmanagement/ajax/createcategory"],
        friend_management_controller::createcategory,
    );
    RouteManager::add_route(
        &["/friendmanagement/ajax/deletecategory"],
        friend_management_controller::deletecategory,
    );
    RouteManager::add_route(
        &["/friendmanagement/ajax/viewcategory"],
        friend_management_controller::view_category,
    );
    RouteManager::add_route(
        &["/friendmanagement/ajax/updatecategoryoptions"],
        friend_management_controller::update_category_options,
    );
    RouteManager::add_route(
        &["/friendmanagement/ajax/movefriends"],
        friend_management_controller::movefriends,
    );
    RouteManager::add_route(
        &["/friendmanagement/ajax/deletefriends"],
        friend_management_controller::deletefriends,
    );

    // Register
    RouteManager::add_route(&["/register"], register_controller::register);
    RouteManager::add_route(
        &["/register/cancel"],
        register_controller::register_cancelled,
    );
    RouteManager::add_route(&["/captcha.jpg"], register_controller::captcha);

    // Habblets
    RouteManager::add_route(
        &["/habblet/ajax/namecheck"],
        name_check_controller::namecheck,
    );
    RouteManager::add_route(
        &["/habblet/ajax/updatemotto"],
        update_motto_controller::updatemotto,
    );
    RouteManager::add_route(
        &["/habblet/ajax/roomselectionCreate"],
        room_selection_controller::create,
    );
    RouteManager::add_route(
        &["/habblet/ajax/roomselectionConfirm"],
        room_selection_controller::confirm,
    );
    RouteManager::add_route(
        &["/habblet/ajax/roomselectionHide"],
        room_selection_controller::hide,
    );
    RouteManager::add_route(
        &["/components/roomNavigation"],
        navigation_component::navigation,
    );
    RouteManager::add_route(&["/habblet/proxy"], proxy_habblet::more_info);
    RouteManager::add_route(
        &["/habboclub/habboclub_confirm"],
        habbo_club_habblet::confirm,
    );
    RouteManager::add_route(
        &["/habboclub/habboclub_subscribe"],
        habbo_club_habblet::subscribe,
    );
    RouteManager::add_route(
        &["/habboclub/habboclub_reminder_remove"],
        habbo_club_habblet::reminder_remove,
    );
    RouteManager::add_route(
        &["/habblet/ajax/habboclub_gift"],
        club_controller::habbo_club_gift,
    );
    RouteManager::add_route(
        &["/habblet/ajax/habboclub_enddate"],
        habbo_club_habblet::enddate,
    );
    RouteManager::add_route(&["/myhabbo/tag/add"], tag_controller::add);
    RouteManager::add_route(&["/myhabbo/tag/remove"], tag_controller::remove);
    RouteManager::add_route(
        &["/habblet/ajax/redeemvoucher"],
        voucher_controller::redeem_voucher,
    );
    RouteManager::add_route(
        &["/remove_all_tags"],
        tag_controller::remove_all_tags,
    );
    RouteManager::add_route(
        &["/habblet/ajax/tagsearch"],
        tag_controller::tagsearch,
    );
    RouteManager::add_route(&["/habblet/ajax/tagfight"], tag_controller::tagfight);
    RouteManager::add_route(&["/habblet/mytagslist"], tag_controller::mytaglist);
    RouteManager::add_route(&["/habblet/ajax/tagmatch"], tag_controller::tagmatch);
    RouteManager::add_route(&["/habblet/ajax/tagmatch"], tag_controller::tagmatch);
    RouteManager::add_route(
        &["/habblet/ajax/collectiblesConfirm"],
        collectables_controller::confirm,
    );
    RouteManager::add_route(
        &["/habblet/ajax/collectiblesPurchase"],
        collectables_controller::purchase,
    );
    RouteManager::add_route(
        &["/habblet/ajax/load_events"],
        event_controller::load_events,
    );
    RouteManager::add_route(
        &["/habblet/ajax/mgmgetinvitelink"],
        invite_controller::invite_link,
    );
    RouteManager::add_route(
        &["/habblet/habbosearchcontent"],
        invite_controller::search_content,
    );
    RouteManager::add_route(
        &["/habblet/ajax/confirmAddFriend"],
        invite_controller::confirm_add_friend,
    );
    RouteManager::add_route(
        &["/habblet/ajax/addFriend"],
        invite_controller::add_friend,
    );
    RouteManager::add_route(
        &["/myhabbo/avatarlist/avatarinfo"],
        friends_widget_controller::avatarinfo,
    );
    RouteManager::add_route(&["/myhabbo/friends/add"], invite_controller::add);
    RouteManager::add_route(&["/habblet/cproxy"], proxy_habblet::minimail);
    RouteManager::add_route(
        &["/habblet/ajax/removeFeedItem"],
        feed_controller::remove_feed_item,
    );
    RouteManager::add_route(&["/habblet/ajax/nextgift"], feed_controller::nextgift);
    RouteManager::add_route(
        &["/habblet/ajax/giftqueueHide"],
        feed_controller::giftqueue_hide,
    );
    RouteManager::add_route(&["/habblet/ajax/clear_hand"], proxy_habblet::clearhand);
    RouteManager::add_route(
        &["/habblet/ajax/token_generate"],
        proxy_habblet::token_generate,
    );
    RouteManager::add_route(
        &["/habblet/ajax/preview_news_article"],
        housekeeping_news_controller::preview_news_article,
    );

    // Groups
    RouteManager::add_route(&["/groups/*/id"], group_controller::view_group);
    RouteManager::add_route(&["/groups/*"], group_controller::view_group);
    RouteManager::add_route(
        &["/grouppurchase/group_create_form"],
        group_habblet_controller::group_create_form,
    );
    RouteManager::add_route(
        &["/grouppurchase/purchase_confirmation"],
        group_habblet_controller::purchase_confirmation,
    );
    RouteManager::add_route(
        &["/grouppurchase/purchase_ajax"],
        group_habblet_controller::purchase_ajax,
    );
    RouteManager::add_route(
        &["/groups/actions/startEditingSession/*"],
        group_controller::start_editing_session,
    );
    RouteManager::add_route(
        &["/groups/actions/cancelEditingSession"],
        group_controller::cancel_editing_session,
    );
    RouteManager::add_route(
        &["/groups/actions/group_settings"],
        group_habblet_controller::group_settings,
    );
    RouteManager::add_route(
        &["/groups/actions/saveEditingSession"],
        group_controller::save_editing_session,
    );
    RouteManager::add_route(
        &["/groups/actions/update_group_settings"],
        group_habblet_controller::update_group_settings,
    );
    RouteManager::add_route(
        &["/groups/actions/check_group_url"],
        group_habblet_controller::check_group_url,
    );
    RouteManager::add_route(
        &["/groups/actions/show_badge_editor"],
        group_habblet_controller::show_badge_editor,
    );
    RouteManager::add_route(
        &["/groups/actions/update_group_badge"],
        group_habblet_controller::update_group_badge,
    );
    RouteManager::add_route(
        &["/groups/actions/confirm_delete_group"],
        group_habblet_controller::confirm_delete_group,
    );
    RouteManager::add_route(
        &["/groups/actions/delete_group"],
        group_habblet_controller::delete_group,
    );
    RouteManager::add_route(
        &["/myhabbo/tag/addgrouptag"],
        group_tag_controller::add_group_tag,
    );
    RouteManager::add_route(
        &["/myhabbo/tag/listgrouptags"],
        group_tag_controller::list_group_tag,
    );
    RouteManager::add_route(
        &["/myhabbo/tag/removegrouptag"],
        group_tag_controller::remove_group_tag,
    );
    RouteManager::add_route(&["/groups/actions/join"], group_member_controller::join);
    RouteManager::add_route(
        &["/groups/actions/confirm_leave"],
        group_member_controller::confirm_leave,
    );
    RouteManager::add_route(&["/groups/actions/leave"], group_member_controller::leave);
    RouteManager::add_route(
        &["/myhabbo/groups/memberlist"],
        group_member_controller::memberlist,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/confirm_revoke_rights"],
        group_member_controller::confirm_revoke_rights,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/revoke_rights"],
        group_member_controller::revoke_rights,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/confirm_give_rights"],
        group_member_controller::confirm_give_rights,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/give_rights"],
        group_member_controller::give_rights,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/confirm_remove"],
        group_member_controller::confirm_remove,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/remove"],
        group_member_controller::remove,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/confirm_accept"],
        group_member_controller::confirm_accept,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/accept"],
        group_member_controller::accept,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/confirm_decline"],
        group_member_controller::confirm_decline,
    );
    RouteManager::add_route(
        &["/myhabbo/groups/batch/decline"],
        group_member_controller::decline,
    );
    RouteManager::add_route(
        &["/myhabbo/avatarlist/membersearchpaging"],
        member_widget_controller::membersearchpaging,
    );
    RouteManager::add_route(
        &["/groups/actions/confirm_select_favorite"],
        group_favourite_controller::confirmselectfavourite,
    );
    RouteManager::add_route(
        &["/groups/actions/select_favorite"],
        group_favourite_controller::selectfavourite,
    );
    RouteManager::add_route(
        &["/groups/actions/confirm_deselect_favorite"],
        group_favourite_controller::confirmdeselectfavourite,
    );
    RouteManager::add_route(
        &["/groups/actions/deselect_favorite"],
        group_favourite_controller::deselectfavourite,
    );

    // Group discussions
    RouteManager::add_route(
        &["/groups/*/id/discussions/page/*"],
        group_discussions_controller::view_discussions_page,
    );
    RouteManager::add_route(
        &["/groups/*/discussions/page/*"],
        group_discussions_controller::view_discussions_page,
    );
    RouteManager::add_route(
        &["/groups/*/id/discussions"],
        group_discussions_controller::view_discussions,
    );
    RouteManager::add_route(
        &["/groups/*/discussions"],
        group_discussions_controller::view_discussions,
    );
    RouteManager::add_route(
        &["/groups/*/id/discussions/*/id"],
        discussion_controller::view_discussion,
    );
    RouteManager::add_route(
        &["/groups/*/discussions/*/id"],
        discussion_controller::view_discussion,
    );
    RouteManager::add_route(
        &["/groups/*/id/discussions/*/id/page/*"],
        discussion_controller::view_discussion,
    );
    RouteManager::add_route(
        &["/groups/*/discussions/*/id/page/*"],
        discussion_controller::view_discussion,
    );
    RouteManager::add_route(
        &["/discussions/actions/pingsession"],
        discussion_actions_controller::pingsession,
    );
    RouteManager::add_route(
        &["/discussions/actions/newtopic"],
        discussion_actions_controller::newtopic,
    );
    RouteManager::add_route(
        &["/discussions/actions/savetopic"],
        discussion_actions_controller::savetopic,
    );
    RouteManager::add_route(
        &["/discussions/actions/previewtopic"],
        discussion_preview_controller::previewtopic,
    );
    RouteManager::add_route(
        &["/discussions/actions/previewpost"],
        discussion_preview_controller::previewpost,
    );
    RouteManager::add_route(
        &["/discussions/actions/opentopicsettings"],
        discussion_actions_controller::opentopicsettings,
    );
    RouteManager::add_route(
        &["/discussions/actions/confirm_delete_topic"],
        discussion_actions_controller::confirm_delete_topic,
    );
    RouteManager::add_route(
        &["/discussions/actions/deletetopic"],
        discussion_actions_controller::deletetopic,
    );
    RouteManager::add_route(
        &["/discussions/actions/savetopicsettings"],
        discussion_actions_controller::savetopicsettings,
    );
    RouteManager::add_route(
        &["/discussions/actions/updatepost"],
        discussion_actions_controller::updatepost,
    );
    RouteManager::add_route(
        &["/discussions/actions/deletepost"],
        discussion_actions_controller::deletepost,
    );
    RouteManager::add_route(
        &["/discussions/actions/savepost"],
        discussion_actions_controller::savepost,
    );

    // Store
    RouteManager::add_route(&["/myhabbo/store/main"], store_controller::main);
    RouteManager::add_route(&["/myhabbo/store/items"], store_controller::items);
    RouteManager::add_route(&["/myhabbo/store/preview"], store_controller::preview);
    RouteManager::add_route(
        &["/myhabbo/store/purchase_confirm"],
        store_controller::purchase_confirm,
    );
    RouteManager::add_route(
        &["/myhabbo/store/background_warning"],
        store_controller::background_warning,
    );
    RouteManager::add_route(
        &["/myhabbo/store/purchase_stickers"],
        store_controller::purchase_stickers,
    );
    RouteManager::add_route(
        &["/myhabbo/store/purchase_backgrounds"],
        store_controller::purchase_backgrounds,
    );
    RouteManager::add_route(
        &["/myhabbo/store/purchase_stickie_notes"],
        store_controller::purchase_stickie_notes,
    );
    RouteManager::add_route(
        &["/myhabbo/sticker/place_sticker"],
        widget_controller::place_sticker,
    );
    RouteManager::add_route(
        &["/myhabbo/sticker/remove_sticker"],
        widget_controller::remove_sticker,
    );
    RouteManager::add_route(
        &["/myhabbo/widget/add"],
        widget_controller::place_widget,
    );
    RouteManager::add_route(
        &["/myhabbo/widget/delete"],
        widget_controller::remove_widget,
    );
    RouteManager::add_route(&["/myhabbo/save"], homes_controller::save);

    // Homes
    RouteManager::add_route(&["/home/*"], homes_controller::home);
    RouteManager::add_route(&["/home/*/id"], homes_controller::home);
    RouteManager::add_route(
        &["/myhabbo/widget/edit"],
        widget_controller::edit_widget,
    );
    RouteManager::add_route(
        &["/myhabbo/store/inventory"],
        homes_controller::inventory,
    );
    RouteManager::add_route(
        &["/myhabbo/store/inventory_items"],
        homes_controller::inventory_items,
    );
    RouteManager::add_route(
        &["/myhabbo/store/inventory_preview"],
        homes_controller::inventory_preview,
    );
    RouteManager::add_route(&["/myhabbo/tag/list"], homes_controller::tag_list);
    RouteManager::add_route(
        &["/myhabbo/noteeditor/editor"],
        note_editor_controller::note_editor,
    );
    RouteManager::add_route(
        &["/myhabbo/noteeditor/preview"],
        note_editor_controller::note_preview,
    );
    RouteManager::add_route(&["/myhabbo/linktool/search"], note_editor_controller::search);
    RouteManager::add_route(&["/myhabbo/noteeditor/place"], note_editor_controller::place);
    RouteManager::add_route(&["/myhabbo/stickie/edit"], note_editor_controller::stickie_edit);
    RouteManager::add_route(
        &["/myhabbo/stickie/delete"],
        note_editor_controller::stickie_delete,
    );
    RouteManager::add_route(
        &["/myhabbo/startSession/*"],
        homes_controller::start_editing_session,
    );
    RouteManager::add_route(
        &["/myhabbo/cancel/*"],
        homes_controller::cancel_editing_session,
    );
    RouteManager::add_route(
        &["/myhabbo/stickie/delete"],
        note_editor_controller::stickie_delete,
    );

    // Widgets
    RouteManager::add_route(&["/myhabbo/rating/rate"], rate_controller::rate);
    RouteManager::add_route(
        &["/myhabbo/rating/reset_ratings"],
        rate_controller::reset_rating,
    );
    RouteManager::add_route(
        &["/myhabbo/badgelist/badgepaging"],
        badges_controller::badge_paging,
    );
    RouteManager::add_route(
        &["/myhabbo/avatarlist/friendsearchpaging"],
        friends_widget_controller::friendsearchpaging,
    );
    RouteManager::add_route(&["/myhabbo/groups/groupinfo"], group_controller::groupinfo);
    RouteManager::add_route(&["/myhabbo/guestbook/preview"], guestbook_controller::preview);
    RouteManager::add_route(&["/myhabbo/guestbook/add"], guestbook_controller::add);
    RouteManager::add_route(&["/myhabbo/guestbook/remove"], guestbook_controller::remove);
    RouteManager::add_route(
        &["/myhabbo/guestbook/configure"],
        guestbook_controller::configure,
    );
    RouteManager::add_route(
        &["/myhabbo/traxplayer/select_song"],
        trax_controller::select_song,
    );
    RouteManager::add_route(&["/trax/song/*"], trax_controller::get_song);

    // Minimail
    RouteManager::add_route(
        &["/minimail/loadMessages"],
        minimail_controller::load_messages,
    );
    RouteManager::add_route(&["/minimail/recipients"], minimail_controller::recipients);
    RouteManager::add_route(&["/minimail/preview"], minimail_controller::preview);
    RouteManager::add_route(
        &["/minimail/sendMessage"],
        minimail_controller::send_message,
    );
    RouteManager::add_route(&["/minimail/loadMessage"], minimail_controller::load_message);
    RouteManager::add_route(
        &["/minimail/deleteMessage"],
        minimail_controller::delete_message,
    );
    RouteManager::add_route(
        &["/minimail/undeleteMessage"],
        minimail_controller::undelete_message,
    );
    RouteManager::add_route(&["/minimail/emptyTrash"], minimail_controller::empty_trash);

    // Quick menu
    RouteManager::add_route(&["/quickmenu/groups"], quickmenu_controller::groups);
    RouteManager::add_route(&["/quickmenu/rooms"], quickmenu_controller::rooms);
    RouteManager::add_route(&["/quickmenu/friends_all"], quickmenu_controller::friends);

    // API
    RouteManager::add_route(
        &["/api/advertisement/get_img"],
        advertisement_controller::get_img,
    );
    RouteManager::add_route(
        &["/api/advertisement/get_url"],
        advertisement_controller::get_url,
    );
    RouteManager::add_route(&["/api/verify/get/*"], verify_controller::get);
    RouteManager::add_route(&["/api/verify/clear/*"], verify_controller::clear);
    RouteManager::add_route(&["/habbo-imaging/*"], imager_controller::imager_redirect);

    // XML Promo habbos
    RouteManager::add_route(
        &["/xml/promo_habbos.xml"],
        xml_controller::promo_habbos,
    );
    RouteManager::add_route(
        &["/xml/promo_habbos_v2.xml"],
        xml_controller::promo_habbos_v2,
    );

    // Housekeeping
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}")],
        housekeeping_controller::dashboard,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/")],
        housekeeping_controller::dashboard,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/login")],
        housekeeping_controller::login,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/api/ban")],
        housekeeping_commands_controller::ban,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/logout")],
        housekeeping_controller::logout,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/transaction/lookup")],
        housekeeping_transactions_controller::search,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/transaction/track_item")],
        housekeeping_transactions_controller::item_lookup,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/users/search")],
        housekeeping_users_controller::search,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/users/create")],
        housekeeping_users_controller::create,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/users/edit")],
        housekeeping_users_controller::edit,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/users/imitate/*")],
        housekeeping_users_controller::imitate,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/infobus_polls")],
        housekeeping_infobus_controller::polls,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/infobus_polls/create")],
        housekeeping_infobus_controller::create_polls,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/infobus_polls/delete")],
        housekeeping_infobus_controller::delete,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/infobus_polls/edit")],
        housekeeping_infobus_controller::edit,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/infobus_polls/view_results")],
        housekeeping_infobus_controller::view_results,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/infobus_polls/clear_results")],
        housekeeping_infobus_controller::clear_results,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/infobus_polls/send_poll")],
        housekeeping_infobus_controller::send_poll,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/infobus_polls/close_event")],
        housekeeping_infobus_controller::close_event,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/infobus_polls/door_status")],
        housekeeping_infobus_controller::door_status,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/articles")],
        housekeeping_news_controller::articles,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/articles/create")],
        housekeeping_news_controller::create,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/articles/delete")],
        housekeeping_news_controller::delete,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/articles/edit")],
        housekeeping_news_controller::edit,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/configurations")],
        housekeeping_config_controller::configurations,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/bans")],
        housekeeping_bans_controller::bans,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/room_ads")],
        housekeeping_ads_controller::roomads,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/room_ads/delete")],
        housekeeping_ads_controller::delete,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/room_ads/create")],
        housekeeping_ads_controller::create,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/room_badges")],
        housekeeping_room_badges_controller::badges,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/room_badges/delete")],
        housekeeping_room_badges_controller::delete,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/room_badges/create")],
        housekeeping_room_badges_controller::create,
    );
    add_route(
        vec![format!("/{HOUSEKEEPING_PATH}/catalogue/edit_frontpage")],
        housekeeping_catalogue_frontpage_controller::edit,
    );
}
