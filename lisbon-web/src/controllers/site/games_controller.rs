//! Mirrors `org.alexdev.http.controllers.site.GamesController`.

use lisbon_server::dao::mysql::highscore_dao::HighscoreDao;
use lisbon_server::game::games::enums::game_type::GameType;

use crate::duckhttpd::Template;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::util::xss_util::XssUtil as XSSUtil;

const HIGHSCORES_LIMIT: i32 = 10;

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|character| character.is_ascii_digit())
}

/// Mirrors `games(WebConnection)`.
pub fn games(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let mut template = web_connection.template("games");

    if !web_connection.session().contains("highscoreGameId") {
        web_connection.session().set("highscoreGameId", SessionValue::Str("1".to_string()));
    }

    let game_id = web_connection.session().get_int("highscoreGameId");

    let game_type = if game_id == 2 {
        GameType::Snowstorm
    } else if game_id == 0 {
        GameType::WobbleSquabble
    } else {
        GameType::Battleball
    };

    web_connection.session().set("gameScoreViewMonthly", SessionValue::Bool(true));

    web_connection.session().set("page", SessionValue::Str("games".to_string()));
    append_personal_highscores(
        &mut template,
        game_type,
        1,
        game_id,
        web_connection.session().get_boolean("gameScoreViewMonthly"),
    );
    template.render();

    Ok(())
}

/// Mirrors `games_all_time(WebConnection)`.
pub fn games_all_time(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let mut template = web_connection.template("games");

    if !web_connection.session().contains("highscoreGameId") {
        web_connection.session().set("highscoreGameId", SessionValue::Str("1".to_string()));
    }

    let game_id = web_connection.session().get_int("highscoreGameId");

    let game_type = if game_id == 2 {
        GameType::Snowstorm
    } else if game_id == 0 {
        GameType::WobbleSquabble
    } else {
        GameType::Battleball
    };

    web_connection.session().set("gameScoreViewMonthly", SessionValue::Bool(false));

    web_connection.session().set("page", SessionValue::Str("games".to_string()));
    append_personal_highscores(
        &mut template,
        game_type,
        1,
        game_id,
        web_connection.session().get_boolean("gameScoreViewMonthly"),
    );
    template.render();

    Ok(())
}

/// Mirrors `appendpersonalhighscores(Template, GameType, int, int, boolean)`.
fn append_personal_highscores(
    template: &mut impl Template,
    game_type: GameType,
    page_number: i32,
    game_id: i32,
    view_monthly: bool,
) {
    let score_entries =
        HighscoreDao::get_scores(HIGHSCORES_LIMIT, game_type, page_number, view_monthly);
    template.set("scoreEntries", TemplateValue::of(&score_entries));
    template.set("gameId", TemplateValue::of(game_id));
    template.set("pageNumber", TemplateValue::of(page_number));
    template.set("viewMonthlyScores", TemplateValue::of(view_monthly));

    let has_next_page =
        !HighscoreDao::get_scores(HIGHSCORES_LIMIT, game_type, page_number + 1, view_monthly)
            .is_empty();

    template.set("hasNextPage", TemplateValue::of(has_next_page));
}

/// Mirrors `personalhighscores(WebConnection)`.
pub fn personalhighscores(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut template = web_connection.template("habblet/personalhighscores");

    let mut page_number = 1;

    if web_connection.post().contains("pageNumber")
        && web_connection
            .post()
            .get_string("pageNumber")
            .map_or(false, |value| is_numeric(&value))
    {
        page_number = web_connection.post().get_int("pageNumber").unwrap_or(1);

        if page_number < 1 {
            page_number = 1;
        }
    }

    let mut game_id = 1;
    let mut game_type = GameType::Battleball;

    if web_connection.post().contains("gameId")
        && web_connection
            .post()
            .get_string("gameId")
            .map_or(false, |value| is_numeric(&value))
    {
        game_id = web_connection.post().get_int("gameId").unwrap_or(1);

        if game_id == 2 {
            game_type = GameType::Snowstorm;
        }

        if game_id == 0 {
            game_type = GameType::WobbleSquabble;
        }

        web_connection
            .session()
            .set("highscoreGameId", SessionValue::Str(game_id.to_string()));
    }

    append_personal_highscores(
        &mut template,
        game_type,
        page_number,
        game_id,
        web_connection.session().get_boolean("gameScoreViewMonthly"),
    );
    template.render();

    Ok(())
}
