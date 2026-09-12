//! Mirrors `net.h4bbo.lisbon.game.player.statistics.PlayerStatistic`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PlayerStatistic {
    DaysLoggedInRow,
    GuestbookUnreadMessages,
    OnlineTime,
    BattleballPointsAllTime,
    SnowstormPointsAllTime,
    WobbleSquabblePointsAllTime,
    BattleballMonthlyScores,
    SnowstormMonthlyScores,
    WobbleSquabbleMonthlyScores,
    XpEarnedMonth,
    XpAllTime,
    BattleballGamesWon,
    SnowstormGamesWon,
    WobbleSquabbleGamesWon,
    GuidedBy,
    HasTutorial,
    IsGuidable,
    PlayersGuided,
    NewbieRoomLayout,
    NewbieGift,
    NewbieGiftTime,
    GiftsDue,
    ClubGiftDue,
    ClubMemberTime,
    ClubMemberTimeUpdated,
    ActivationCode,
    ForgotPasswordCode,
    ForgotRecoveryRequestedTime,
    MuteExpiresAt,
}

impl PlayerStatistic {
    /// Mirrors `getColumn()`.
    pub fn column(&self) -> &'static str {
        match self {
            PlayerStatistic::DaysLoggedInRow => "days_logged_in_row",
            PlayerStatistic::GuestbookUnreadMessages => "guestbook_unread_messages",
            PlayerStatistic::OnlineTime => "online_time",
            PlayerStatistic::BattleballPointsAllTime => "battleball_score_all_time",
            PlayerStatistic::SnowstormPointsAllTime => "snowstorm_score_all_time",
            PlayerStatistic::WobbleSquabblePointsAllTime => "wobble_squabble_score_all_time",
            PlayerStatistic::BattleballMonthlyScores => "battleball_score_month",
            PlayerStatistic::SnowstormMonthlyScores => "snowstorm_score_month",
            PlayerStatistic::WobbleSquabbleMonthlyScores => "wobble_squabble_score_month",
            PlayerStatistic::XpEarnedMonth => "xp_earned_month",
            PlayerStatistic::XpAllTime => "xp_all_time",
            PlayerStatistic::BattleballGamesWon => "battleball_games_won",
            PlayerStatistic::SnowstormGamesWon => "snowstorm_games_won",
            PlayerStatistic::WobbleSquabbleGamesWon => "wobble_squabble_games_won",
            PlayerStatistic::GuidedBy => "guided_by",
            PlayerStatistic::HasTutorial => "has_tutorial",
            PlayerStatistic::IsGuidable => "is_guidable",
            PlayerStatistic::PlayersGuided => "players_guided",
            PlayerStatistic::NewbieRoomLayout => "newbie_room_layout",
            PlayerStatistic::NewbieGift => "newbie_gift",
            PlayerStatistic::NewbieGiftTime => "newbie_gift_time",
            PlayerStatistic::GiftsDue => "gifts_due",
            PlayerStatistic::ClubGiftDue => "club_gift_due",
            PlayerStatistic::ClubMemberTime => "club_member_time",
            PlayerStatistic::ClubMemberTimeUpdated => "club_member_time_updated",
            PlayerStatistic::ActivationCode => "activation_code",
            PlayerStatistic::ForgotPasswordCode => "forgot_password_code",
            PlayerStatistic::ForgotRecoveryRequestedTime => "forgot_recovery_requested_time",
            PlayerStatistic::MuteExpiresAt => "mute_expires_at",
        }
    }

    /// Mirrors `isDateTime()`.
    pub fn is_date_time(&self) -> bool {
        matches!(self, PlayerStatistic::ClubGiftDue)
    }
}
