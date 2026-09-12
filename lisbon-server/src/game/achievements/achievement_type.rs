//! Mirrors `net.h4bbo.lisbon.game.achievements.AchievementType`.

use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::progressions::achievement_avatar_looks::AchievementAvatarLooks;
use crate::game::achievements::progressions::achievement_email_verification::AchievementEmailVerification;
use crate::game::achievements::progressions::achievement_game_played::AchievementGamePlayed;
use crate::game::achievements::progressions::achievement_graduate::AchievementGraduate;
use crate::game::achievements::progressions::achievement_guide::AchievementGuide;
use crate::game::achievements::progressions::achievement_habbo_club::AchievementHabboClub;
use crate::game::achievements::progressions::achievement_happy_hour::AchievementHappyHour;
use crate::game::achievements::progressions::achievement_login::AchievementLogin;
use crate::game::achievements::progressions::achievement_motto::AchievementMotto;
use crate::game::achievements::progressions::achievement_registration_duration::AchievementRegistrationDuration;
use crate::game::achievements::progressions::achievement_room_entry::AchievementRoomEntry;
use crate::game::achievements::progressions::achievement_student::AchievementStudent;
use crate::game::achievements::progressions::achievement_tags::AchievementTags;

static ACHIEVEMENT_AVATAR_LOOKS: AchievementAvatarLooks = AchievementAvatarLooks;
static ACHIEVEMENT_HABBO_CLUB: AchievementHabboClub = AchievementHabboClub;
static ACHIEVEMENT_MOTTO: AchievementMotto = AchievementMotto;
static ACHIEVEMENT_TAGS: AchievementTags = AchievementTags;
static ACHIEVEMENT_GRADUATE: AchievementGraduate = AchievementGraduate;
static ACHIEVEMENT_HAPPY_HOUR: AchievementHappyHour = AchievementHappyHour;
static ACHIEVEMENT_REGISTRATION_DURATION: AchievementRegistrationDuration = AchievementRegistrationDuration;
static ACHIEVEMENT_ROOM_ENTRY: AchievementRoomEntry = AchievementRoomEntry;
static ACHIEVEMENT_LOGIN: AchievementLogin = AchievementLogin;
static ACHIEVEMENT_GAME_PLAYED: AchievementGamePlayed = AchievementGamePlayed;
static ACHIEVEMENT_GUIDE: AchievementGuide = AchievementGuide;
static ACHIEVEMENT_STUDENT: AchievementStudent = AchievementStudent;
static ACHIEVEMENT_EMAIL_VERIFICATION: AchievementEmailVerification = AchievementEmailVerification;

/// Mirrors the `AchievementType` enum (the commented-out Java variants —
/// `RespectGiven`, `MGM`, `TraderPass`, `AIPerformanceVote`, `RespectEarned`,
/// `AllTimeHotelPresence` — are omitted to match the Java source).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AchievementType {
    AchievementLooks,
    Hc,
    Motto,
    Tags,
    Graduate,
    HappyHour,
    RegistrationDuration,
    RoomEntry,
    Login,
    GamePlayed,
    Guide,
    Student,
    EmailVerification,
}

impl AchievementType {
    /// Mirrors `values()`.
    pub fn values() -> [AchievementType; 13] {
        [
            AchievementType::AchievementLooks,
            AchievementType::Hc,
            AchievementType::Motto,
            AchievementType::Tags,
            AchievementType::Graduate,
            AchievementType::HappyHour,
            AchievementType::RegistrationDuration,
            AchievementType::RoomEntry,
            AchievementType::Login,
            AchievementType::GamePlayed,
            AchievementType::Guide,
            AchievementType::Student,
            AchievementType::EmailVerification,
        ]
    }

    /// Mirrors `getByName(String)`.
    pub fn get_by_name(name: &str) -> Option<AchievementType> {
        Self::values().into_iter().find(|t| t.get_name() == name)
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &'static str {
        match self {
            AchievementType::AchievementLooks => "ACH_AvatarLooks",
            AchievementType::Hc => "HC",
            AchievementType::Motto => "ACH_Motto",
            AchievementType::Tags => "ACH_AvatarTags",
            AchievementType::Graduate => "ACH_Graduate",
            AchievementType::HappyHour => "ACH_HappyHour",
            AchievementType::RegistrationDuration => "ACH_RegistrationDuration",
            AchievementType::RoomEntry => "ACH_RoomEntry",
            AchievementType::Login => "ACH_Login",
            AchievementType::GamePlayed => "ACH_GamePlayed",
            AchievementType::Guide => "GL",
            AchievementType::Student => "ACH_Student",
            AchievementType::EmailVerification => "ACH_EmailVerification",
        }
    }

    /// Mirrors `getProgressor()`.
    pub fn get_progressor(&self) -> &'static dyn AchievementProgress {
        match self {
            AchievementType::AchievementLooks => &ACHIEVEMENT_AVATAR_LOOKS,
            AchievementType::Hc => &ACHIEVEMENT_HABBO_CLUB,
            AchievementType::Motto => &ACHIEVEMENT_MOTTO,
            AchievementType::Tags => &ACHIEVEMENT_TAGS,
            AchievementType::Graduate => &ACHIEVEMENT_GRADUATE,
            AchievementType::HappyHour => &ACHIEVEMENT_HAPPY_HOUR,
            AchievementType::RegistrationDuration => &ACHIEVEMENT_REGISTRATION_DURATION,
            AchievementType::RoomEntry => &ACHIEVEMENT_ROOM_ENTRY,
            AchievementType::Login => &ACHIEVEMENT_LOGIN,
            AchievementType::GamePlayed => &ACHIEVEMENT_GAME_PLAYED,
            AchievementType::Guide => &ACHIEVEMENT_GUIDE,
            AchievementType::Student => &ACHIEVEMENT_STUDENT,
            AchievementType::EmailVerification => &ACHIEVEMENT_EMAIL_VERIFICATION,
        }
    }

    /// Mirrors `hasRemovePreviousAchievement()`.
    pub fn has_remove_previous_achievement(&self) -> bool {
        match self {
            AchievementType::Guide => false,
            _ => true,
        }
    }
}
