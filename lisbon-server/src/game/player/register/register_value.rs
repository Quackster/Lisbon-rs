//! Mirrors `net.h4bbo.lisbon.game.player.register.RegisterValue`.
use crate::game::player::register::register_data_type::RegisterDataType;

#[derive(Clone, Debug)]
pub struct RegisterValue {
    label: String,
    data_type: RegisterDataType,
    value: String,
    flag: bool,
}

impl RegisterValue {
    /// Mirrors the `RegisterValue(String, RegisterDataType)` constructor.
    pub fn new(label: &str, data_type: RegisterDataType) -> Self {
        Self {
            label: label.to_string(),
            data_type,
            value: String::new(),
            flag: false,
        }
    }

    /// Mirrors `getLabel`.
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Mirrors `getDataType`.
    pub fn get_data_type(&self) -> RegisterDataType {
        self.data_type
    }

    /// Mirrors `getFlag`.
    pub fn get_flag(&self) -> bool {
        self.flag
    }

    /// Mirrors `setFlag`.
    pub fn set_flag(&mut self, flag: bool) {
        self.flag = flag;
    }

    /// Mirrors `getValue`.
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Mirrors `setValue`.
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }
}
