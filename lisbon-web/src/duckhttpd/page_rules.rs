//! Mirrors `org.alexdev.duckhttpd.routes.PageRules` (external Java library).

use std::sync::{Mutex, OnceLock};

use regex::Regex;

/// Mirrors `PageRules.PageRule`.
#[derive(Clone)]
pub struct PageRule {
    pattern: String,
    redirection: String,
}

/// Mirrors `org.alexdev.duckhttpd.routes.PageRules`.
pub struct PageRules {
    page_rule_list: Mutex<Vec<PageRule>>,
    blacklist: Mutex<Vec<String>>,
}

impl PageRules {
    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static PageRules {
        static INSTANCE: OnceLock<PageRules> = OnceLock::new();

        INSTANCE.get_or_init(|| {
            PageRules {
                page_rule_list: Mutex::new(Vec::new()),
                blacklist: Mutex::new(Vec::new()),
            }
        })
    }

    /// Mirrors `addRule(String, String)`.
    pub fn add_rule(&self, pattern: &str, redirection: &str) {
        self.page_rule_list.lock().unwrap().push(PageRule {
            pattern: pattern.replace('*', "(.*?)"),
            redirection: redirection.to_string(),
        });
    }

    /// Mirrors `addBlacklist(String)`.
    pub fn add_blacklist(&self, pattern: &str) {
        self.blacklist
            .lock()
            .unwrap()
            .push(pattern.replace('*', "(.*?)"));
    }

    /// Mirrors `matchesRule(String)`.
    pub fn matches_rule(&self, url: &str) -> Option<PageRule> {
        for pattern in self.blacklist.lock().unwrap().iter() {
            if Self::matches(pattern, url) {
                return None;
            }
        }

        for rule in self.page_rule_list.lock().unwrap().iter() {
            if Self::matches(&rule.pattern, url) {
                return Some(rule.clone());
            }
        }

        None
    }

    /// Mirrors `getNewUrl(PageRule, String)`.
    pub fn get_new_url(&self, rule: &PageRule, url: &str) -> String {
        let mut new_url = rule.redirection.clone();

        if let Ok(regex) = Regex::new(&format!("^(?:{})$", rule.pattern)) {
            if let Some(captures) = regex.captures(url) {
                let mut index = 1;

                for group in captures.iter().skip(1) {
                    if let Some(group) = group {
                        new_url = new_url.replace(&format!("${index}"), group.as_str());
                    }

                    index += 1;
                }
            }
        }

        new_url
    }

    fn matches(pattern: &str, url: &str) -> bool {
        Regex::new(&format!("^(?:{})$", pattern))
            .map(|regex| regex.is_match(url))
            .unwrap_or(false)
    }
}
