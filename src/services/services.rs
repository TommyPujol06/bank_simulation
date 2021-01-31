use crate::services::base::Service;
use crate::walker_alias::AliasTable;
use rand::thread_rng;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Services {
    POS,
    ATM,
    WEB,
    APP,
    BOT,
    HHD,
    OFF,
}

impl Services {
    // Define the probability of each service to be chosen at random.
    pub const _POS_P: f64 = 05.0;
    pub const _ATM_P: f64 = 15.0;
    pub const _WEB_P: f64 = 30.0;
    pub const _APP_P: f64 = 15.0;
    pub const _BOT_P: f64 = 05.0;
    pub const _HHD_P: f64 = 10.0;
    pub const _OFF_P: f64 = 20.0;

    pub fn get_probability(&self) -> f64 {
        match self {
            Services::POS => Services::_POS_P,
            Services::ATM => Services::_ATM_P,
            Services::WEB => Services::_WEB_P,
            Services::APP => Services::_APP_P,
            Services::BOT => Services::_BOT_P,
            Services::HHD => Services::_HHD_P,
            Services::OFF => Services::_OFF_P,
        }
    }

    pub fn random(ptable: &AliasTable, services: &Vec<Service>) -> Service {
        let rng = thread_rng();
        let idx = ptable.random(rng);
        services[idx]
    }
}
