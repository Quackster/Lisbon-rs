//! Mirrors `org.alexdev.http.game.stickers.StickerManager`.

use std::sync::OnceLock;

use crate::dao::homes_dao::HomesDao;
use crate::dao::store_dao::StoreDao;
use crate::dao::widget_dao::WidgetDao;
use crate::game::homes::widget::Widget;
use crate::game::stickers::sticker_category::StickerCategory;
use crate::game::stickers::sticker_product::StickerProduct;
use crate::game::stickers::sticker_type::StickerType;

pub struct StickerManager {
    catalogue_list: Vec<StickerProduct>,
    category_list: Vec<StickerCategory>,
}

impl StickerManager {
    /// Mirrors the `StickerManager()` constructor.
    fn new() -> Self {
        Self {
            category_list: StoreDao::get_categories(),
            catalogue_list: StoreDao::get_catalogue(),
        }
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static StickerManager {
        static INSTANCE: OnceLock<StickerManager> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Mirrors `createHome(int)`.
    /// Port note: a missing `getStickerByData` result yields the sticker id
    // `0` instead of the Java NPE.
    pub fn create_home(&self, user_id: i32) {
        HomesDao::create(user_id);

        WidgetDao::purchase_widget(user_id, 455, 27, 129, 1, self.sticker_id("profilewidget", StickerType::HomeWidget), "", 0, true);
        WidgetDao::purchase_widget(user_id, 440, 321, 177, 1, self.sticker_id("roomswidget", StickerType::HomeWidget), "", 0, true);
        WidgetDao::purchase_widget(user_id, 383, 491, 179, 6, self.sticker_id("highscoreswidget", StickerType::HomeWidget), "", 0, true);
        WidgetDao::purchase_widget(user_id, 183, 371, 171, 1, self.sticker_id("paper_clip_1", StickerType::Sticker), "", 0, true);
        WidgetDao::purchase_widget(user_id, 109, 19, 134, 1, self.sticker_id("needle_3", StickerType::Sticker), "", 0, true);
        WidgetDao::purchase_widget(user_id, 281, 346, 150, 1, self.sticker_id("sticker_spaceduck", StickerType::Sticker), "", 0, true);
        WidgetDao::purchase_widget(user_id, 56, 229, 151, 2, self.sticker_id("stickienote", StickerType::Note), "Welcome to a brand new Habbo Home page!\nThis is the place where you can express yourself with a wild and unique variety of stickers, hoot yo\ntrap off with colourful notes and showcase your Habbo rooms! To\nstart editing just click the edit button.\n", 0, true);
        WidgetDao::purchase_widget(user_id, 110, 409, 170, 5, self.sticker_id("stickienote", StickerType::Note), "Where are my friends?\nTo add your buddy list to your page click edit and look in your widgets inventory. After placing it on the page you can move it all over the place and even change how it looks. Go on!", 0, true);
        WidgetDao::purchase_widget(user_id, 125, 38, 131, 4, self.sticker_id("stickienote", StickerType::Note), "Remember!\nPosting personal information about yourself or your friends, including addresses, phone numbers or email, and getting round the filter will result in your note being deleted.\nDeleted notes will not be funded.\n", 0, true);
        WidgetDao::purchase_widget(user_id, 0, 0, 0, 1, self.sticker_id("guestbookwidget", StickerType::HomeWidget), "", 0, false);
        WidgetDao::purchase_widget(user_id, 0, 0, 0, 1, self.sticker_id("badgeswidget", StickerType::HomeWidget), "", 0, false);
        WidgetDao::purchase_widget(user_id, 0, 0, 0, 1, self.sticker_id("friendswidget", StickerType::HomeWidget), "", 0, false);
        WidgetDao::purchase_widget(user_id, 0, 0, 0, 1, self.sticker_id("groupswidget", StickerType::HomeWidget), "", 0, false);
        WidgetDao::purchase_widget(user_id, 0, 0, 0, 1, self.sticker_id("traxplayerwidget", StickerType::HomeWidget), "", 0, false);
        WidgetDao::purchase_widget(user_id, 0, 0, 0, 1, self.sticker_id("ratingwidget", StickerType::HomeWidget), "", 0, false);
    }

    fn sticker_id(&self, data: &str, sticker_type: StickerType) -> i32 {
        self.get_sticker_by_data(data, sticker_type)
            .map(|product| product.id)
            .unwrap_or(0)
    }

    /// Mirrors `getDefaultWidgets(int)`.
    pub fn get_default_widgets(&self, user_id: i32) -> Vec<Widget> {
        let mut widgets = Vec::new();

        widgets.push(Widget::new(1, user_id, 455, 27, 129, self.sticker_id("profilewidget", StickerType::HomeWidget), 1, 1, "", 1, true, None));
        widgets.push(Widget::new(2, user_id, 440, 321, 177, self.sticker_id("roomswidget", StickerType::HomeWidget), 6, 1, "", 1, true, None));
        widgets.push(Widget::new(3, user_id, 383, 491, 179, self.sticker_id("highscoreswidget", StickerType::HomeWidget), 1, 1, "", 1, true, None));
        widgets.push(Widget::new(4, user_id, 183, 371, 171, self.sticker_id("paper_clip_1", StickerType::Sticker), 1, 1, "", 1, true, None));
        widgets.push(Widget::new(5, user_id, 109, 19, 134, self.sticker_id("needle_3", StickerType::Sticker), 1, 1, "", 1, true, None));
        widgets.push(Widget::new(6, user_id, 281, 346, 150, self.sticker_id("sticker_spaceduck", StickerType::Sticker), 2, 1, "", 1, true, None));

        widgets.push(Widget::new(7, user_id, 56, 229, 151, self.sticker_id("stickienote", StickerType::Note), 2, 1, "Welcome to a brand new Habbo Home page!\nThis is the place where you can express yourself with a wild and unique variety of stickers, hoot yo\ntrap off with colourful notes and showcase your Habbo rooms! To\nstart editing just click the edit button.\n", 1, true, None));

        widgets.push(Widget::new(8, user_id, 110, 409, 170, self.sticker_id("stickienote", StickerType::Note), 5, 1, "To add your buddy list to your page click edit and look in your widgets inventory. After placing it on the page you can move it all over the place and even change how it looks. Go on!", 1, true, None));

        widgets.push(Widget::new(9, user_id, 125, 38, 131, self.sticker_id("stickienote", StickerType::Note), 4, 1, "Remember!\nPosting personal information about yourself or your friends, including addresses, phone numbers or email, and getting round the filter will result in your note being deleted.\nDeleted notes will not be funded.\n", 1, true, None));

        widgets
    }

    /// Mirrors `getCatalogueList()`.
    pub fn get_catalogue_list(&self) -> Vec<StickerProduct> {
        self.catalogue_list.clone()
    }

    /// Mirrors `getCategories(int)`.
    pub fn get_categories(&self, min_rank: i32) -> Vec<StickerCategory> {
        self.category_list
            .iter()
            .filter(|category| min_rank >= category.min_rank)
            .cloned()
            .collect()
    }

    /// Mirrors `getCategory(int)`.
    pub fn get_category(&self, id: i32) -> Option<StickerCategory> {
        self.category_list
            .iter()
            .find(|category| category.id == id)
            .cloned()
    }

    /// Mirrors `getStickerProduct(int)`.
    pub fn get_sticker_product(&self, sticker_id: i32) -> Option<StickerProduct> {
        self.catalogue_list
            .iter()
            .find(|product| product.id == sticker_id)
            .cloned()
    }

    /// Mirrors `getSkin(int)`.
    pub fn get_skin(&self, skin_id: i32) -> String {
        match skin_id {
            1 => "defaultskin".to_string(),
            2 => "speechbubbleskin".to_string(),
            3 => "metalskin".to_string(),
            4 => "noteitskin".to_string(),
            5 => "notepadskin".to_string(),
            6 => "goldenskin".to_string(),
            7 => "hc_machineskin".to_string(),
            8 => "hc_pillowskin".to_string(),
            _ => "nakedskin".to_string(),
        }
    }

    /// Mirrors `getStickerByData(String, StickerType)`.
    pub fn get_sticker_by_data(&self, data: &str, sticker_type: StickerType) -> Option<StickerProduct> {
        self.catalogue_list
            .iter()
            .find(|product| {
                product.data.eq_ignore_ascii_case(data) && product.get_type() == Some(sticker_type)
            })
            .cloned()
    }
}
