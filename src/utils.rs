use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

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
    file.read_to_string(&mut contents).expect("Can't read file content to string");
    let nb = contents.parse().expect("Can't parse file contents to u128");
    dbg!(path, nb);
    nb
}

pub fn write_u128(val: u128, path: &str) {
    // 写入字符串到文件
    let mut output_file = File::create(path).expect("Can't create file");
    let data = val.to_string();
    output_file.write_all(data.as_bytes()).expect("Can't write file");
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
    let u = read_u128("testfile");
    dbg!(&u);

}
