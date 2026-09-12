//! Mirrors `org.alexdev.http.template.binders.SiteBinder`.

use lisbon_server::util::config::game_configuration::GameConfiguration;

use crate::duckhttpd::{Template, TemplateBinder, TemplateValue, WebConnection};
use crate::routes::HOUSEKEEPING_PATH;
use crate::server::watchdog::{IS_SERVER_ONLINE, LAST_VISITS, USERS_ONLNE};
use crate::util::captcha::Captcha;

/// Mirrors `org.alexdev.http.template.binders.SiteBinder`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SiteBinder {
    #[serde(rename = "siteName")]
    site_name: String,
    #[serde(rename = "sitePath")]
    site_path: String,

    #[serde(rename = "loaderGameIp")]
    loader_game_ip: String,
    #[serde(rename = "loaderGamePort")]
    loader_game_port: String,

    #[serde(rename = "loaderMusIp")]
    loader_mus_ip: String,
    #[serde(rename = "loaderMusPort")]
    loader_mus_port: String,

    #[serde(rename = "loaderDcr")]
    loader_dcr: String,
    #[serde(rename = "loaderVariables")]
    loader_variables: String,
    #[serde(rename = "loaderTexts")]
    loader_texts: String,

    #[serde(rename = "usersOnline")]
    users_online: i32,
    #[serde(rename = "serverOnline")]
    server_online: bool,
    #[serde(rename = "formattedUsersOnline")]
    formatted_users_online: String,

    #[serde(rename = "visits")]
    visits: i32,
    #[serde(rename = "housekeepingPath")]
    housekeeping_path: String,
    #[serde(rename = "staticContentPath")]
    static_content_path: String,

    #[serde(rename = "loaderFlashBase")]
    loader_flash_base: String,
    #[serde(rename = "loaderFlashSwf")]
    loader_flash_swf: String,
    #[serde(rename = "loaderFlashTexts")]
    loader_flash_texts: String,
    #[serde(rename = "loaderFlashVariables")]
    loader_flash_variables: String,

    #[serde(rename = "loaderFlashBetaBase")]
    loader_flash_beta_base: String,
    #[serde(rename = "loaderFlashBetaSwf")]
    loader_flash_beta_swf: String,
    #[serde(rename = "loaderFlashBetaTexts")]
    loader_flash_beta_texts: String,
    #[serde(rename = "loaderFlashBetaVariables")]
    loader_flash_beta_variables: String,

    #[serde(rename = "emailSiteName")]
    email_site_name: String,
    #[serde(rename = "emailHotelName")]
    email_hotel_name: String,
    #[serde(rename = "emailStaticPath")]
    email_static_path: String,
    #[serde(rename = "furniImagerPath")]
    furni_imager_path: String,

    captcha: Captcha,
}

impl SiteBinder {
    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        Self {
            site_name: String::new(),
            site_path: String::new(),
            loader_game_ip: String::new(),
            loader_game_port: String::new(),
            loader_mus_ip: String::new(),
            loader_mus_port: String::new(),
            loader_dcr: String::new(),
            loader_variables: String::new(),
            loader_texts: String::new(),
            users_online: 0,
            server_online: false,
            formatted_users_online: String::new(),
            visits: 0,
            housekeeping_path: String::new(),
            static_content_path: String::new(),
            loader_flash_base: String::new(),
            loader_flash_swf: String::new(),
            loader_flash_texts: String::new(),
            loader_flash_variables: String::new(),
            loader_flash_beta_base: String::new(),
            loader_flash_beta_swf: String::new(),
            loader_flash_beta_texts: String::new(),
            loader_flash_beta_variables: String::new(),
            email_site_name: String::new(),
            email_hotel_name: String::new(),
            email_static_path: String::new(),
            furni_imager_path: String::new(),
            captcha: Captcha::new(),
        }
    }

    /// Mirrors `getSiteName()`.
    pub fn get_site_name(&self) -> &str {
        &self.site_name
    }

    /// Mirrors `getSitePath()`.
    pub fn get_site_path(&self) -> &str {
        &self.site_path
    }

    /// Mirrors `getLoaderGameIp()`.
    pub fn get_loader_game_ip(&self) -> &str {
        &self.loader_game_ip
    }

    /// Mirrors `getLoaderGamePort()`.
    pub fn get_loader_game_port(&self) -> &str {
        &self.loader_game_port
    }

    /// Mirrors `getLoaderMusIp()`.
    pub fn get_loader_mus_ip(&self) -> &str {
        &self.loader_mus_ip
    }

    /// Mirrors `getLoaderMusPort()`.
    pub fn get_loader_mus_port(&self) -> &str {
        &self.loader_mus_port
    }

    /// Mirrors `getLoaderDcr()`.
    pub fn get_loader_dcr(&self) -> &str {
        &self.loader_dcr
    }

    /// Mirrors `getLoaderVariables()`.
    pub fn get_loader_variables(&self) -> &str {
        &self.loader_variables
    }

    /// Mirrors `getLoaderTexts()`.
    pub fn get_loader_texts(&self) -> &str {
        &self.loader_texts
    }

    /// Mirrors `getUsersOnline()`.
    pub fn get_users_online(&self) -> i32 {
        self.users_online
    }

    /// Mirrors `isServerOnline()`.
    pub fn is_server_online(&self) -> bool {
        self.server_online
    }

    /// Mirrors `getFormattedUsersOnline()`.
    pub fn get_formatted_users_online(&self) -> &str {
        &self.formatted_users_online
    }

    /// Mirrors `getVisits()`.
    pub fn get_visits(&self) -> i32 {
        self.visits
    }

    /// Mirrors `getHousekeepingPath()`.
    pub fn get_housekeeping_path(&self) -> &str {
        &self.housekeeping_path
    }

    /// Mirrors `getStaticContentPath()`.
    pub fn get_static_content_path(&self) -> &str {
        &self.static_content_path
    }

    /// Mirrors `getLoaderFlashBase()`.
    pub fn get_loader_flash_base(&self) -> &str {
        &self.loader_flash_base
    }

    /// Mirrors `getLoaderFlashSwf()`.
    pub fn get_loader_flash_swf(&self) -> &str {
        &self.loader_flash_swf
    }

    /// Mirrors `getLoaderFlashTexts()`.
    pub fn get_loader_flash_texts(&self) -> &str {
        &self.loader_flash_texts
    }

    /// Mirrors `getLoaderFlashVariables()`.
    pub fn get_loader_flash_variables(&self) -> &str {
        &self.loader_flash_variables
    }

    /// Mirrors `getLoaderFlashBetaBase()`.
    pub fn get_loader_flash_beta_base(&self) -> &str {
        &self.loader_flash_beta_base
    }

    /// Mirrors `getLoaderFlashBetaSwf()`.
    pub fn get_loader_flash_beta_swf(&self) -> &str {
        &self.loader_flash_beta_swf
    }

    /// Mirrors `getLoaderFlashBetaTexts()`.
    pub fn get_loader_flash_beta_texts(&self) -> &str {
        &self.loader_flash_beta_texts
    }

    /// Mirrors `getLoaderFlashBetaVariables()`.
    pub fn get_loader_flash_beta_variables(&self) -> &str {
        &self.loader_flash_beta_variables
    }

    /// Mirrors `getEmailSiteName()`.
    pub fn get_email_site_name(&self) -> &str {
        &self.email_site_name
    }

    /// Mirrors `getEmailHotelName()`.
    pub fn get_email_hotel_name(&self) -> &str {
        &self.email_hotel_name
    }

    /// Mirrors `getEmailStaticPath()`.
    pub fn get_email_static_path(&self) -> &str {
        &self.email_static_path
    }

    /// Mirrors `getFurniImagerPath()`.
    pub fn get_furni_imager_path(&self) -> &str {
        &self.furni_imager_path
    }

    /// Mirrors `getCaptcha()`.
    pub fn get_captcha(&self) -> &Captcha {
        &self.captcha
    }

    /// Mirrors `NumberFormat.getNumberInstance(Locale.US).format(int)`.
    fn format_number(value: i32) -> String {
        let digits = (value as i64).unsigned_abs().to_string();
        let mut out = String::new();
        for (index, ch) in digits.chars().enumerate() {
            if index > 0 && (digits.len() - index) % 3 == 0 {
                out.push(',');
            }
            out.push(ch);
        }
        if value < 0 {
            out.insert(0, '-');
        }
        out
    }

    /// Mirrors `StringUtils.capitalise(String)` (plexus).
    fn capitalise(value: &str) -> String {
        let mut chars = value.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }
}

impl TemplateBinder for SiteBinder {
    /// Mirrors `onRegister(Template, WebConnection)`.
    fn on_register(
        &mut self,
        template: &mut dyn Template,
        _web_connection: Option<&WebConnection>,
    ) {
        let config = GameConfiguration::get_instance();

        self.captcha = Captcha::new();

        self.site_name = config.get_string("site.name");
        self.site_path = config.get_string("site.path");
        self.static_content_path = config.get_string("static.content.path");
        self.furni_imager_path = "https://classichabbo.com/imager/furni".to_string();

        self.email_static_path = config.get_string("email.static.content.path");
        self.email_hotel_name = Self::capitalise(
            &config
                .get_string("site.path")
                .replace("https://", "")
                .replace("http://", "")
                .replace("/", ""),
        );

        self.loader_game_ip = config.get_string("loader.game.ip");
        self.loader_game_port = config.get_string("loader.game.port");

        self.loader_mus_ip = config.get_string("loader.mus.ip");
        self.loader_mus_port = config.get_string("loader.mus.port");

        self.loader_dcr = config.get_string("loader.dcr");
        self.loader_variables = config.get_string("loader.external.variables");
        self.loader_texts = config.get_string("loader.external.texts");

        self.loader_flash_base = config.get_string("loader.flash.base");
        self.loader_flash_swf = config.get_string("loader.flash.swf");
        self.loader_flash_texts = config.get_string("loader.flash.external.texts");
        self.loader_flash_variables =
            config.get_string("loader.flash.external.variables");

        self.loader_flash_beta_base = config.get_string("loader.flash.beta.base");
        self.loader_flash_beta_swf = config.get_string("loader.flash.beta.swf");
        self.loader_flash_beta_texts = config.get_string("loader.flash.beta.external.texts");
        self.loader_flash_beta_variables =
            config.get_string("loader.flash.beta.external.variables");

        self.server_online = IS_SERVER_ONLINE.load(std::sync::atomic::Ordering::SeqCst);
        self.users_online = USERS_ONLNE.load(std::sync::atomic::Ordering::SeqCst);
        self.formatted_users_online = Self::format_number(self.users_online);

        self.visits = LAST_VISITS.load(std::sync::atomic::Ordering::SeqCst);
        self.housekeeping_path = HOUSEKEEPING_PATH.to_string();

        template.set("site", TemplateValue::of(self.clone()));
        let game_config = GameConfiguration::get_instance();
        template.set(
            "gameConfig",
            TemplateValue::json(serde_json::to_value(&*game_config).unwrap_or(serde_json::Value::Null)),
        );
    }
}
