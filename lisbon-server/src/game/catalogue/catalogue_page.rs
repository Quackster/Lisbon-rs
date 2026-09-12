//! Mirrors `net.h4bbo.lisbon.game.catalogue.CataloguePage`.

use std::collections::HashMap;

use crate::game::player::player_rank::PlayerRank;
use crate::util::string_util::StringUtil;

#[derive(Clone, Debug)]
pub struct CataloguePage {
    id: i32,
    min_role: PlayerRank,
    index_visible: bool,
    is_club_only: bool,
    name_index: String,
    link_list: String,
    name: String,
    layout: String,
    image_headline: String,
    image_teasers: Option<String>,
    body: Option<String>,
    label_pick: String,
    label_extra_s: String,
    label_extra: HashMap<String, String>,
}

impl CataloguePage {
    /// Mirrors the 14-arg `CataloguePage` constructor.
    pub fn new(
        id: i32,
        min_role: PlayerRank,
        index_visible: bool,
        is_club_only: bool,
        name_index: &str,
        linked_list: &str,
        name: &str,
        layout: &str,
        image_headline: &str,
        image_teasers: Option<&str>,
        body: Option<&str>,
        label_pick: &str,
        label_extra_s: &str,
        label_extra_t: Option<&str>,
    ) -> Self {
        let mut label_extra = HashMap::new();

        if !StringUtil::is_null_or_empty(label_extra_t) {
            for db_extra_data in label_extra_t.unwrap().split("\r\n") {
                let extra_data = StringUtil::filter_input(db_extra_data, true);

                if let Some((extra_id, data)) = extra_data.split_once(':') {
                    label_extra.insert(
                        format!("label_extra_t_{extra_id}"),
                        data.to_string(),
                    );
                }
            }
        }

        Self {
            id,
            min_role,
            index_visible,
            is_club_only,
            name_index: name_index.to_string(),
            link_list: linked_list.to_string(),
            name: name.to_string(),
            layout: layout.to_string(),
            image_headline: image_headline.to_string(),
            image_teasers: image_teasers.map(String::from),
            body: body.map(String::from),
            label_pick: label_pick.to_string(),
            label_extra_s: label_extra_s.to_string(),
            label_extra,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `setId(int)`.
    pub fn set_id(&mut self, id: i32) {
        self.id = id
    }

    /// Mirrors `getMinRole()`.
    pub fn get_min_role(&self) -> PlayerRank {
        self.min_role
    }

    /// Mirrors `isIndexVisible()`.
    pub fn is_index_visible(&self) -> bool {
        self.index_visible
    }

    /// Mirrors `getNameIndex()`.
    pub fn get_name_index(&self) -> &str {
        &self.name_index
    }

    /// Mirrors `getLinkList()`.
    pub fn get_link_list(&self) -> &str {
        &self.link_list
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `setName(String)`.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Mirrors `getLayout()`.
    pub fn get_layout(&self) -> &str {
        &self.layout
    }

    /// Mirrors `getImageHeadline()`.
    pub fn get_image_headline(&self) -> &str {
        &self.image_headline
    }

    /// Mirrors `getImageTeasers()`.
    pub fn get_image_teasers(&self) -> &str {
        self.image_teasers.as_deref().unwrap_or("")
    }

    /// Mirrors `getBody()`.
    pub fn get_body(&self) -> &str {
        self.body.as_deref().unwrap_or("")
    }

    /// Mirrors `setBody(String)`.
    pub fn set_body(&mut self, body: Option<String>) {
        self.body = body;
    }

    /// Mirrors `getLabelPick()`.
    pub fn get_label_pick(&self) -> &str {
        &self.label_pick
    }

    /// Mirrors `getLabelExtra_s()`.
    pub fn get_label_extra_s(&self) -> &str {
        &self.label_extra_s
    }

    /// Mirrors `getLabelExtra()`.
    pub fn get_label_extra(&self) -> HashMap<String, String> {
        self.label_extra.clone()
    }

    /// Mirrors `isClubOnly()`.
    pub fn is_club_only(&self) -> bool {
        self.is_club_only
    }
}
