//! Mirrors `org.alexdev.http.dao.WidgetDao`.

use sqlx::mysql::MySqlRow;

use lisbon_server::dao::storage::{RowGetters, Storage};

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub use crate::game::homes::widget::Widget;

pub struct WidgetDao;

impl WidgetDao {
    /// Mirrors `getHomeWidgets(int)`.
    pub fn get_home_widgets(user_id: i32) -> Vec<Widget> {
        let mut widgets = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_stickers WHERE user_id = {user_id} AND group_id = 0"))
        {
            widgets.push(Self::fill(&row));
        }

        widgets
    }

    /// Mirrors `getHomeWidgets(int, boolean)`.
    pub fn get_home_widgets_by_placement(user_id: i32, is_placed: bool) -> Vec<Widget> {
        let mut widgets = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM cms_stickers WHERE user_id = {user_id} AND group_id = 0 AND is_placed = {p}", p = is_placed as i32),
        ) {
            widgets.push(Self::fill(&row));
        }

        widgets
    }

    /// Mirrors `getGroupWidgets(int)`.
    pub fn get_group_widgets(group_id: i32) -> Vec<Widget> {
        let mut widgets = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_stickers WHERE group_id = {group_id}"))
        {
            widgets.push(Self::fill(&row));
        }

        widgets
    }

    /// Mirrors `getGroupWidgets(int, boolean)`.
    pub fn get_group_widgets_by_placement(group_id: i32, is_placed: bool) -> Vec<Widget> {
        let mut widgets = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM cms_stickers WHERE group_id = {group_id} AND is_placed = {p}", p = is_placed as i32),
        ) {
            widgets.push(Self::fill(&row));
        }

        widgets
    }

    /// Mirrors `getInventoryWidgets(int, int)`.
    pub fn get_inventory_widgets_by_type(user_id: i32, type_id: i32) -> Vec<Widget> {
        let mut widgets = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM cms_stickers INNER JOIN cms_stickers_catalogue ON cms_stickers_catalogue.id = cms_stickers.sticker_id WHERE cms_stickers.user_id = {user_id} AND cms_stickers_catalogue.type = {type_id} AND cms_stickers.group_id = 0 AND is_placed = 0"),
        ) {
            widgets.push(Self::fill(&row));
        }

        widgets
    }

    /// Mirrors `getInventoryWidgets(int)`.
    pub fn get_inventory_widgets(user_id: i32) -> Vec<Widget> {
        let mut widgets = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM cms_stickers WHERE user_id = {user_id} AND group_id = 0 AND is_placed = 0"),
        ) {
            widgets.push(Self::fill(&row));
        }

        widgets
    }

    /// Mirrors `purchaseWidget(int, int, int, int, int, int, String, int, boolean)`.
    pub fn purchase_widget(user_id: i32, x: i32, y: i32, z: i32, skin_id: i32, sticker_id: i32, text: &str, group_id: i32, is_placed: bool) -> Widget {
        let widget_id = Storage::get_storage()
            .execute_insert(&format!(
                "INSERT INTO cms_stickers (user_id, x, y, z, skin_id, sticker_id, text, group_id, is_placed) VALUES ({user_id}, {x}, {y}, {z}, {skin_id}, {sticker_id}, '{t}', {group_id}, {p})",
                t = escape(text),
                p = is_placed as i32
            ))
            .map(|id| id as i32)
            .unwrap_or(0);

        Widget::new(widget_id, user_id, x, y, z, sticker_id, skin_id, group_id, text, 1, is_placed, None)
    }

    /// Mirrors `save(Widget)`.
    pub fn save(widget: &Widget) {
        let extra_data = match &widget.extra_data {
            Some(data) => format!("'{}'", escape(data)),
            None => "NULL".to_string(),
        };

        Storage::get_storage().execute(&format!(
            "UPDATE cms_stickers SET group_id = {g}, x = {x}, y = {y}, z = {z}, skin_id = {s}, text = '{t}', is_placed = {p}, extra_data = {e} WHERE id = {id}",
            g = widget.group_id,
            x = widget.x,
            y = widget.y,
            z = widget.z,
            s = widget.skin_id,
            t = escape(&widget.text),
            p = widget.is_placed as i32,
            e = extra_data,
            id = widget.id
        ));
    }

    /// Mirrors `delete(int, int)`.
    pub fn delete(widget_id: i32, group_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM cms_stickers WHERE group_id = {group_id} AND id = {widget_id}"
        ));
    }

    /// Mirrors `deleteHomeNote(int, int)`.
    pub fn delete_home_note(stickie_id: i32, user_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM cms_stickers WHERE group_id = 0 AND is_placed = 1 AND user_id = {user_id} AND id = {stickie_id}"
        ));
    }

    /// Mirrors `getGroupWidget(int, int)`.
    pub fn get_group_widget(widget_id: i32, group_id: i32) -> Option<Widget> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_stickers WHERE group_id = {group_id} AND id = {widget_id}"))
        {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `getInventoryWidget(int, int)`.
    pub fn get_inventory_widget(user_id: i32, widget_id: i32) -> Option<Widget> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_stickers WHERE id = {widget_id} AND user_id = {user_id} AND is_placed = 0"))
        {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `getHomeWidget(int, int)`.
    pub fn get_home_widget(user_id: i32, widget_id: i32) -> Option<Widget> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_stickers WHERE id = {widget_id} AND user_id = {user_id} AND group_id = 0"))
        {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `getWidget(int)`.
    pub fn get_widget(widget_id: i32) -> Option<Widget> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_stickers WHERE id = {widget_id}"))
        {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `fill(ResultSet)`.
    fn fill(row: &MySqlRow) -> Widget {
        let text = row.str("text").unwrap_or_default();

        Widget::new(
            row.i32("id").unwrap_or(0),
            row.i32("user_id").unwrap_or(0),
            row.i32("x").unwrap_or(0),
            row.i32("y").unwrap_or(0),
            row.i32("z").unwrap_or(0),
            row.i32("sticker_id").unwrap_or(0),
            row.i32("skin_id").unwrap_or(0),
            row.i32("group_id").unwrap_or(0),
            &text,
            1,
            row.bool("is_placed").unwrap_or(false),
            row.str("extra_data"),
        )
    }
}
