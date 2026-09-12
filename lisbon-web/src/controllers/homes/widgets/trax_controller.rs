//! Mirrors `org.alexdev.http.controllers.homes.widgets.TraxController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::song_machine_dao::SongMachineDao;

use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::stickers::sticker_type::StickerType;

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|character| character.is_ascii_digit())
}

/// Mirrors `selectSong(WebConnection)`.
pub fn select_song(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let Some(player_details) = PlayerDao::get_details(web_connection.session().get_int("user.id"))
    else {
        // Java NPEs on a null player.
        return Ok(());
    };

    let mut widget_id = -1;
    let mut song_id = -1;

    if let Some(value) = web_connection.post().get_int("widgetId") {
        widget_id = value;
    }

    if let Some(value) = web_connection.post().get_int("songId") {
        song_id = value;
    }

    let Some(mut widget) = WidgetDao::get_widget(widget_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if !widget
        .get_product()
        .map(|product| product.data.to_lowercase() == "traxplayerwidget")
        .unwrap_or(false)
    {
        web_connection.send_string("");
        return Ok(());
    }

    let mut can_select = false;

    let product_type = widget.get_product().and_then(|product| product.get_type());

    if product_type == Some(StickerType::GroupWidget) {
        can_select = GroupDao::get_group_owner(widget.group_id) == player_details.get_id();
    } else if product_type == Some(StickerType::HomeWidget) {
        can_select = widget.user_id == player_details.get_id();
    }

    if !can_select {
        web_connection.send_string("");
        return Ok(());
    }

    let song_list = widget.get_songs();
    let song = SongMachineDao::get_song(song_id);

    let is_valid = match &song {
        Some(song) => song_id != 0 && song_list.iter().any(|s| s.get_id() == song.get_id()),
        None => false,
    };

    if !is_valid {
        // Mirrors Java `widget.setExtraData("")`.
        widget.set_extra_data(Some(String::new()));
    } else {
        // Mirrors Java `widget.setExtraData("" + song.getId())`.
        widget.set_extra_data(Some(song.unwrap().get_id().to_string()));
    }

    WidgetDao::save(&widget);

    let mut template = web_connection.template("homes/widget/habblet/trax_song");
    template.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `getSong(WebConnection)`.
pub fn get_song(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let matches = web_connection.get_matches();
    let Some(song_data) = matches.first() else {
        web_connection.send_string("");
        return Ok(());
    };

    if !is_numeric(song_data) {
        web_connection.send_string("");
        return Ok(());
    }

    let Some(song) = SongMachineDao::get_song(song_data.parse().unwrap_or(0)) else {
        web_connection.send_string("");
        return Ok(());
    };

    let data = song.get_data();

    if data.is_empty() {
        // Java `substring(0, length - 1)` throws on empty data.
        web_connection.send_string("");
        return Ok(());
    }

    let data = &data[..data.len() - 1];

    let mut track_data = data.to_string();
    track_data = track_data.replace(":4:", "&track4=");
    track_data = track_data.replace(":3:", "&track3=");
    track_data = track_data.replace(":2:", "&track2=");
    track_data = track_data.replace("1:", "&track1=");

    let author = PlayerDao::get_name(song.get_user_id()).unwrap_or_default();
    web_connection.send_string(
        format!("status=0&name={}&author={}{}", song.get_title(), author, track_data),
    );
    Ok(())
}
