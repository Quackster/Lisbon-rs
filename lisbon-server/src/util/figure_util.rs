//! Mirrors `net.h4bbo.lisbon.util.FigureUtil`.
//!
//! Java parses `figuredata.xml` with a DOM builder; here we use `roxmltree`.

use rand::Rng;
use roxmltree::Document;

pub struct FigureUtil;

impl FigureUtil {
    /// Mirrors `getRandomFigure`.
    pub fn get_random_figure(gender_req: Option<&str>, club_req: bool) -> String {
        let mut figure_output = String::new();

        let gender_req = match gender_req {
            Some(g) => g.to_string(),
            None => {
                if rand::thread_rng().gen_bool(0.5) {
                    "M".to_string()
                } else {
                    "F".to_string()
                }
            }
        };

        let xml = std::fs::read_to_string("figuredata.xml").expect("figuredata.xml not found");
        let doc = Document::parse(&xml).expect("invalid XML in figuredata.xml");

        for set_type in doc.descendants().filter(|n| n.has_tag_name("settype")) {
            let mandatory = set_type
                .attribute("mandatory")
                .map(|v| v == "1")
                .unwrap_or(false);
            let is_mandatory = mandatory || rand::thread_rng().gen_bool(0.5);

            if is_mandatory {
                let typ = set_type.attribute("type").unwrap_or("");
                let mut has_colour = false;
                let mut possible_sets: Vec<(String, bool)> = Vec::new();

                for set in set_type.children().filter(|n| n.has_tag_name("set")) {
                    // If the `gender` attribute is absent, Java skips the set.
                    let gender = set.attribute("gender");
                    match gender {
                        None => continue,
                        Some(g) => {
                            if g != "U" && g != gender_req {
                                continue;
                            }
                        }
                    }

                    if set.attribute("selectable") == Some("0") {
                        continue;
                    }

                    if let Some(colorable) = set.attribute("colorable") {
                        has_colour = colorable == "0";
                    }

                    if set.attribute("club") == Some("1") && !club_req {
                        continue;
                    }

                    let id = set.attribute("id").unwrap_or("");
                    possible_sets.push((id.to_string(), has_colour));
                }

                if possible_sets.is_empty() {
                    continue;
                }

                let index = rand::thread_rng().gen_range(0..possible_sets.len());
                let set_id = possible_sets[index].0.clone();
                let is_colouring_allowed = possible_sets[index].1;

                let mut random_colours: Vec<String> = Vec::new();

                if !is_colouring_allowed {
                    let palette_id = set_type.attribute("paletteid").unwrap_or("");
                    for palette in doc.descendants().filter(|n| n.has_tag_name("palette")) {
                        if palette.attribute("id") != Some(palette_id) {
                            continue;
                        }
                        for set in palette.children().filter(|n| n.has_tag_name("color")) {
                            if set.attribute("club") == Some("1") && !club_req {
                                continue;
                            }
                            if set.attribute("selectable") == Some("0") {
                                continue;
                            }
                            if let Some(id) = set.attribute("id") {
                                random_colours.push(id.to_string());
                            }
                        }
                    }
                }

                figure_output.push_str(typ);
                figure_output.push('-');
                figure_output.push_str(&set_id);
                figure_output.push('-');

                if !random_colours.is_empty() {
                    let c = rand::thread_rng().gen_range(0..random_colours.len());
                    figure_output.push_str(&random_colours[c]);
                }

                figure_output.push('.');
            }
        }

        // Strip the trailing '.' (Java: substring(0, len-1)).
        if figure_output.ends_with('.') {
            figure_output.pop();
        }

        figure_output
    }
}
