use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "serverName")]
    pub server_name: String,

    #[serde(rename = "adminPassword")]
    pub admin_password: String,

    #[serde(rename = "carGroup")]
    pub car_group: String,

    #[serde(rename = "trackMedalsRequirement")]
    pub track_medals_requirement: i64,

    #[serde(rename = "safetyRatingRequirement")]
    pub safety_rating_requirement: i64,

    #[serde(rename = "racecraftRatingRequirement")]
    pub racecraft_rating_requirement: i64,

    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "maxCarSlots")]
    pub max_car_slots: i64,

    #[serde(rename = "spectatorPassword")]
    pub spectator_password: String,

    #[serde(rename = "configVersion")]
    pub config_version: i64,
}
