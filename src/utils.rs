use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Ok};
use near_crypto::InMemorySigner;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::{
    transaction::{Action, FunctionCallAction, Transaction},
    types::BlockReference,
    views::QueryResponseKind,
};
use serde_json::json;
use tokio::time;

use crate::post::render_post_json;

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

pub fn read_u128(path: &str) -> u128 {
    let mut file = File::open(path).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can't read file content to string");
    let pure_contents = contents.replace("\n", "");
    let nb = pure_contents
        .parse()
        .expect("Can't parse file contents to u128");
    dbg!(path, nb);
    nb
}

pub fn write_u128(val: u128, path: &str) {
    // 写入字符串到文件
    let mut output_file = File::create(path).expect("Can't create file");
    let data = val.to_string();
    output_file
        .write_all(data.as_bytes())
        .expect("Can't write file");
}

pub fn convert_near_amount_to_human_readable(value: u128) -> String {
    let value_str = value.to_string();
    let len = value_str.len();

    if len <= 24 {
        let mut fractional_part = &value_str[len - 24..];
        if fractional_part.len() > 2 {
            fractional_part = &fractional_part[0..2]
        }
        let formatted_decimal = format!("0.{} N", fractional_part);
        return formatted_decimal;
    }

    let whole_part = &value_str[0..len - 24];

    let fractional_part = &value_str[len - 24..len - 22];
    if fractional_part.starts_with("00") {
        return format!("{} N", whole_part);
    }
    format!("{}.{} N", whole_part, fractional_part)
}

pub fn convert_oct_amount_to_human_readable(value: u128) -> String {
    let value_str = value.to_string();
    let len = value_str.len();

    if len <= 18 {
        let mut fractional_part = &value_str[len - 18..];
        if fractional_part.len() > 2 {
            fractional_part = &fractional_part[0..2]
        }
        let formatted_decimal = format!("0.{} Oct", fractional_part);
        return formatted_decimal;
    }

    let whole_part = &value_str[0..len - 18];

    let fractional_part = &value_str[len - 18..len - 16];
    if fractional_part.starts_with("00") {
        return format!("{} Oct", whole_part);
    }
    format!("{}.{} Oct", whole_part, fractional_part)
}

#[test]
fn test_convert_near() {
    let value1 = 1000000000000000000000000;
    let value2 = 1000000000000000000000000000;
    let value3 = 100000000000000000000000_u128;

    let result1 = convert_near_amount_to_human_readable(value1);
    let result2 = convert_near_amount_to_human_readable(value2);
    let result3 = convert_near_amount_to_human_readable(value3);

    println!("{}", result1); // Output: 1.00
    println!("{}", result2); // Output: 0.10
    println!("{}", result3); // Output: 0.10
}

#[test]
pub fn test_time() {
    dbg!(&half_hour_ago_timestamp().to_string());
}

#[test]
pub fn test_write_file() {
    write_u128(1703766782635931621, "testfile");
}

#[test]
pub fn test_read_file() {
    let u = read_u128("nano_timestamp");
    dbg!(&u);
}
