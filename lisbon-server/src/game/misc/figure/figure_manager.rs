//! Mirrors `net.h4bbo.lisbon.game.misc.figure.FigureManager`.

use std::collections::HashMap;
use std::sync::OnceLock;

use roxmltree::{Document, NodeType};

use crate::game::misc::figure::figure_color::FigureColor;
use crate::game::misc::figure::figure_part::FigurePart;
use crate::game::misc::figure::figure_set::FigureSet;
use crate::game::misc::figure::figure_set_type::FigureSetType;

const FIGUREDATA_FILE: &str = "figuredata.xml";

#[derive(Clone, Debug)]
pub struct FigureManager {
    figure_palettes: HashMap<i32, Vec<FigureColor>>,
    figure_set_types: HashMap<String, FigureSetType>,
    figure_sets: HashMap<String, FigureSet>,
}

impl FigureManager {
    fn new() -> Self {
        let mut manager = Self {
            figure_palettes: HashMap::new(),
            figure_set_types: HashMap::new(),
            figure_sets: HashMap::new(),
        };

        manager.load_figure_palettes();
        manager.load_figure_set_types();
        manager.load_figure_sets();

        manager
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static FigureManager {
        static INSTANCE: OnceLock<FigureManager> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Mirrors `validateFigureCode(String, String, boolean)`.
    pub fn validate_figure_code(&self, figure: &str, gender: &str, has_club: bool) -> i32 {
        let figure_data: Vec<&str> = figure.split('.').collect();

        if figure_data.is_empty() {
            return 1;
        }

        let mut sets: Vec<&str> = Vec::new();

        for data in &figure_data {
            let parts: Vec<&str> = data.split('-').collect();

            if parts.len() < 2 || parts.len() > 3 {
                return 2;
            }

            sets.push(parts[0]);
        }

        for figure_set_type in self.figure_set_types.values() {
            if figure_set_type.get_set().eq_ignore_ascii_case("sh") {
                continue;
            }

            if figure_set_type.is_mandatory()
                && !sets.iter().any(|set| *set == figure_set_type.get_set())
            {
                return 3;
            }
        }

        for data in &figure_data {
            let parts: Vec<&str> = data.split('-').collect();

            if parts.len() < 2 || parts.len() > 3 {
                return 4;
            }

            let set = parts[0];
            let set_id = parts[1];

            let figure_set = self
                .figure_sets
                .values()
                .find(|s| {
                    s.get_type().eq_ignore_ascii_case(set)
                        && s.get_id().eq_ignore_ascii_case(set_id)
                        && (s.get_gender().eq_ignore_ascii_case(gender)
                            || s.get_gender().eq_ignore_ascii_case("U"))
                })
                .cloned();

            let figure_set = match figure_set {
                Some(figure_set) => figure_set,
                None => return 5,
            };

            if figure_set.is_club() && !has_club {
                return 6;
            }

            if !figure_set.is_selectable() {
                return 7;
            }

            if parts.len() > 2 && !parts[2].is_empty() {
                let palette_id = parts[2];

                if let Some(figure_set_type) = self.figure_set_types.get(set) {
                    if let Some(palette) = self
                        .figure_palettes
                        .get(&figure_set_type.get_palette_id())
                    {
                        if !palette
                            .iter()
                            .any(|p| p.get_colour_id().eq_ignore_ascii_case(palette_id))
                        {
                            return 8;
                        }
                    }
                }
            }
        }

        0
    }

    /// Mirrors `validateFigure(String, String, boolean)`.
    pub fn validate_figure(&self, figure: &str, gender: &str, has_club: bool) -> bool {
        let figure_data: Vec<&str> = figure.split('.').collect();

        if figure_data.is_empty() {
            return false;
        }

        let mut sets: Vec<&str> = Vec::new();

        for data in &figure_data {
            let parts: Vec<&str> = data.split('-').collect();

            if parts.len() < 2 || parts.len() > 3 {
                return false;
            }

            sets.push(parts[0]);
        }

        for figure_set_type in self.figure_set_types.values() {
            if figure_set_type.get_set().eq_ignore_ascii_case("sh") {
                continue;
            }

            if figure_set_type.is_mandatory()
                && !sets.iter().any(|set| *set == figure_set_type.get_set())
            {
                return false;
            }
        }

        for data in &figure_data {
            let parts: Vec<&str> = data.split('-').collect();

            if parts.len() < 2 || parts.len() > 3 {
                return false;
            }

            let set = parts[0];
            let set_id = parts[1];

            let figure_set = self
                .figure_sets
                .values()
                .find(|s| {
                    s.get_type().eq_ignore_ascii_case(set)
                        && s.get_id().eq_ignore_ascii_case(set_id)
                        && (s.get_gender().eq_ignore_ascii_case(gender)
                            || s.get_gender().eq_ignore_ascii_case("U"))
                })
                .cloned();

            let figure_set = match figure_set {
                Some(figure_set) => figure_set,
                None => return false,
            };

            if figure_set.is_club() && !has_club {
                return false;
            }

            if !figure_set.is_selectable() {
                return false;
            }

            if parts.len() > 2 && !parts[2].is_empty() {
                let palette_id = parts[2];

                if let Some(figure_set_type) = self.figure_set_types.get(set) {
                    if let Some(palette) = self
                        .figure_palettes
                        .get(&figure_set_type.get_palette_id())
                    {
                        if !palette
                            .iter()
                            .any(|p| p.get_colour_id().eq_ignore_ascii_case(palette_id))
                        {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    /// Mirrors `loadFigurePalettes()`.
    fn load_figure_palettes(&mut self) {
        let doc = match Self::parse_doc(FIGUREDATA_FILE) {
            Some(doc) => doc,
            None => return,
        };

        for node in doc
            .descendants()
            .filter(|n| n.has_tag_name("colors"))
            .collect::<Vec<_>>()
        {
            let palette_id = match node
                .attribute("id")
                .and_then(|value| value.parse::<i32>().ok())
            {
                Some(palette_id) => palette_id,
                None => continue,
            };

            let mut palette: Vec<FigureColor> = Vec::new();

            for colour in node.children().filter(|n| {
                n.node_type() == NodeType::Element
            }) {
                let colour_id = colour.attribute("id").unwrap_or("").to_string();
                let index = colour.attribute("index").unwrap_or("").to_string();
                let is_club_required = colour
                    .attribute("club")
                    .map(|value| value.eq_ignore_ascii_case("1"))
                    .unwrap_or(false);
                let is_selectable = colour
                    .attribute("selectable")
                    .map(|value| value.eq_ignore_ascii_case("1"))
                    .unwrap_or(false);

                palette.push(FigureColor::new(
                    colour_id,
                    index,
                    is_club_required,
                    is_selectable,
                ));
            }

            self.figure_palettes.insert(palette_id, palette);
        }
    }

    /// Mirrors `loadFigureSetTypes()`.
    fn load_figure_set_types(&mut self) {
        let doc = match Self::parse_doc(FIGUREDATA_FILE) {
            Some(doc) => doc,
            None => return,
        };

        for node in doc
            .descendants()
            .filter(|n| n.has_tag_name("settype"))
            .collect::<Vec<_>>()
        {
            let set = node.attribute("type").unwrap_or("").to_string();
            let palette_id = match node
                .attribute("paletteid")
                .and_then(|value| value.parse::<i32>().ok())
            {
                Some(palette_id) => palette_id,
                None => continue,
            };
            let is_mandatory = node
                .attribute("mandatory")
                .map(|value| value.eq_ignore_ascii_case("1"))
                .unwrap_or(false);

            self.figure_set_types.insert(
                set.clone(),
                FigureSetType::new(set, palette_id, is_mandatory),
            );
        }
    }

    /// Mirrors `loadFigureSets()`.
    fn load_figure_sets(&mut self) {
        let doc = match Self::parse_doc(FIGUREDATA_FILE) {
            Some(doc) => doc,
            None => return,
        };

        for node in doc
            .descendants()
            .filter(|n| n.has_tag_name("set"))
            .collect::<Vec<_>>()
        {
            let set_type = node
                .parent()
                .and_then(|parent| parent.attribute("type"))
                .unwrap_or("")
                .to_string();
            let id = node.attribute("id").unwrap_or("").to_string();
            let gender = node.attribute("gender").unwrap_or("").to_string();
            let club = node
                .attribute("club")
                .map(|value| value.eq_ignore_ascii_case("1"))
                .unwrap_or(false);
            let colorable = node
                .attribute("colorable")
                .map(|value| value.eq_ignore_ascii_case("1"))
                .unwrap_or(false);
            let selectable = node
                .attribute("selectable")
                .map(|value| value.eq_ignore_ascii_case("1"))
                .unwrap_or(false);

            let mut figure_set = FigureSet::new(
                set_type,
                id.clone(),
                gender,
                club,
                colorable,
                selectable,
            );

            for part in node
                .children()
                .filter(|n| n.node_type() == NodeType::Element)
            {
                if part.has_tag_name("hiddenlayers") {
                    continue;
                }

                figure_set.add_figure_part(FigurePart::new(
                    part.attribute("id").unwrap_or("").to_string(),
                    part.attribute("type").unwrap_or("").to_string(),
                    part.attribute("colorable")
                        .map(|value| value.eq_ignore_ascii_case("1"))
                        .unwrap_or(false),
                    part.attribute("index")
                        .and_then(|value| value.parse::<i32>().ok())
                        .unwrap_or(0),
                ));
            }

            self.figure_sets.insert(id, figure_set);
        }
    }

    /// Loads `figuredata.xml` (Java's `DocumentBuilder.parse(new File(...))` with a
    /// silent exception catch).
    fn parse_doc(file: &str) -> Option<Document<'static>> {
        let xml = std::fs::read_to_string(file).ok()?;
        let xml: &'static str = Box::leak(xml.into_boxed_str());
        Document::parse(xml).ok()
    }
}
