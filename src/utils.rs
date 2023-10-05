use std::{
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::*;

pub mod u128_dec_format {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub fn half_hour_ago_timestamp() -> u128 {
    let now = SystemTime::now();
    let earlier = now
        .checked_sub(std::time::Duration::from_secs(30 * 60))
        .unwrap();
    earlier
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos()
}

pub fn get_dir_path(account_id: &str) -> PathBuf {
    let mut home_dir = dirs::home_dir().expect("Impossible to get your home dir!");
    home_dir.push(format!(".near-credentials/mainnet/{}.json", account_id));
    home_dir
}

#[test]
pub fn test_time() {
    dbg!(&half_hour_ago_timestamp().to_string());
}
