//! Mirrors `net.h4bbo.lisbon.game.alerts.AlertType`.

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize)]
pub enum AlertType {
    HcExpired,
    Present,
    TutorScore,
    CreditDonation,
}
