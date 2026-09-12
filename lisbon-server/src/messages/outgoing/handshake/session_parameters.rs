//! Mirrors `net.h4bbo.lisbon.messages.outgoing.handshake.SESSION_PARAMETERS`.
use crate::game::player::player_details::PlayerDetails;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;
use crate::util::config::game_configuration::GameConfiguration;

/// Mirrors the nested `SESSION_PARAMETERS.SessionParamType` enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SessionParamType {
    RegisterCoppa,
    VoucherEnabled,
    RegisterRequireParentEmail,
    RegisterSendParentEmail,
    AllowDirectMail,
    DateFormat,
    PartnerIntegrationEnabled,
    AllowProfileEditing,
    TrackingHeader,
    TutorialEnabled,
}

impl SessionParamType {
    /// Mirrors `getParamID()`.
    pub fn get_param_id(&self) -> i32 {
        match self {
            Self::RegisterCoppa => 0,
            Self::VoucherEnabled => 1,
            Self::RegisterRequireParentEmail => 2,
            Self::RegisterSendParentEmail => 3,
            Self::AllowDirectMail => 4,
            Self::DateFormat => 5,
            Self::PartnerIntegrationEnabled => 6,
            Self::AllowProfileEditing => 7,
            Self::TrackingHeader => 8,
            Self::TutorialEnabled => 9,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SESSION_PARAMETERS {
    #[allow(dead_code)]
    details: PlayerDetails,
}

impl SESSION_PARAMETERS {
    /// Mirrors the `SESSION_PARAMETERS(PlayerDetails)` constructor.
    pub fn new(details: PlayerDetails) -> Self {
        Self { details }
    }

    /// Mirrors `isTutorialEnabled()`.
    fn is_tutorial_enabled() -> bool {
        GameConfiguration::get_instance().get_bool("tutorial.enabled")
    }
}

impl MessageComposer for SESSION_PARAMETERS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        let config = GameConfiguration::get_instance();

        let mut parameters: Vec<(i32, String)> = Vec::new();
        parameters.push((
            SessionParamType::VoucherEnabled.get_param_id(),
            if config.get_bool("vouchers.enabled") {
                "1".to_string()
            } else {
                "0".to_string()
            },
        ));
        parameters.push((
            SessionParamType::RegisterRequireParentEmail.get_param_id(),
            "0".to_string(),
        ));
        parameters.push((
            SessionParamType::RegisterSendParentEmail.get_param_id(),
            "0".to_string(),
        ));
        parameters.push((
            SessionParamType::AllowDirectMail.get_param_id(),
            "0".to_string(),
        ));
        parameters.push((
            SessionParamType::DateFormat.get_param_id(),
            "dd-MM-yyyy".to_string(),
        ));
        parameters.push((
            SessionParamType::PartnerIntegrationEnabled.get_param_id(),
            "0".to_string(),
        ));
        parameters.push((
            SessionParamType::AllowProfileEditing.get_param_id(),
            if config.get_bool("profile.editing") {
                "1".to_string()
            } else {
                "0".to_string()
            },
        ));
        parameters.push((
            SessionParamType::TrackingHeader.get_param_id(),
            String::new(),
        ));
        parameters.push((
            SessionParamType::TutorialEnabled.get_param_id(),
            if Self::is_tutorial_enabled() {
                "1".to_string()
            } else {
                "0".to_string()
            },
        ));

        response.write_int(parameters.len() as i32);

        for (param_id, value) in &parameters {
            response.write_int(*param_id);

            if value.len() > 0
                && value
                    .chars()
                    .next()
                    .map_or(false, |first| first.is_ascii_digit())
            {
                response.write_int(value.parse::<i32>().unwrap_or(0));
            } else {
                response.write_string(value.as_str());
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        257
    }
}
