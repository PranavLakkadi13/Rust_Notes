use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
mod json;

#[derive(Serialize, Deserialize, Debug)]
// #[serde(deny_unknown_fields)] //  --> used when the struct has less variables inside than those in the JSON being parsed
// #[serde(rename_all="PascalCase")]
// by deafult it prints the json value in the same way they are mentioned in the struct to add more casing look above
// normal result --> {"name":"doggy","year_born":2003}
// PascalCase result --> {"Name":"doggy","YearBorn":2003}
struct Dog {
    name: String,
    year_born: u16,
    owner: DogOwner,
}

#[derive(Deserialize, Serialize, Debug)]
struct DogOwner {
    first_name: String,
    last_name: String,
}

fn main() {
    let dog1 = Dog {
        name: "doggy".to_string(),
        year_born: 2003,
        owner: DogOwner {
            first_name: "Lakkadi".to_string(),
            last_name: "Reddy".to_string(),
        },
    };

    // here the to_string() is a method taken from the serialized library to serialize(convert) data
    let y = to_string_pretty(&dog1);

    if y.is_ok() {
        // std::println!("{}", y.ok().unwrap());
    }

    // deserialze_data();
    deserialze_data2();
}

#[allow(dead_code)]
fn deserialze_data() {
    let json_string = r#"
    {
        "name": "doggy",
        "year_born": 2003,
        "owner": {
            "first_name": "Lakkadi",
            "last_name": "Reddy"
        },
        "breed": "DOG"
    }
    "#;

    // here we use "<>" after "from_str" bcoz we need to tell the rust compiler about which datatype to deserialize it to
    let read_json_data = from_str::<Dog>(json_string);

    if read_json_data.is_ok() {
        println!("{:#?}", read_json_data.ok().unwrap());
    } else {
        std::println!("{:#?}",read_json_data.err());
    }
}

fn deserialze_data2() {
    let json_data = json::test();

    let read_json_data = serde_json::from_str::<RootEarnDetails>(&json_data);

    if read_json_data.is_ok() {
        println!("{:#?}", read_json_data.ok().unwrap());
    } else {
        std::println!("{:#?}",read_json_data.err());
    }
}




use serde::Deserializer;
use std::str::FromStr;

// Helper functions for custom deserialization
fn string_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    u128::from_str(s).map_err(serde::de::Error::custom)
}

fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    f64::from_str(s).map_err(serde::de::Error::custom)
}

// Struct for Pool data
#[derive(Deserialize, Debug)]
pub struct PoolData {
    pub pool: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub assetLiquidityFees: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub runeLiquidityFees: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub totalLiquidityFeesRune: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub saverEarning: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub rewards: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub earnings: f64,
}

// Struct for Meta data
#[derive(Deserialize, Debug)]
pub struct RunePoolMeta {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub liquidityFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub blockRewards: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub earnings: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub bondingEarnings: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub liquidityEarnings: u128,
    #[serde(deserialize_with = "string_to_f64")]
    pub avgNodeCount: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub runePriceUSD: f64,
    pub pools: Vec<PoolData>,
}

// Struct for Interval data
#[derive(Deserialize, Debug)]
pub struct RunePoolInterval {
    #[serde(deserialize_with = "string_to_u128")]
    pub startTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub endTime: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub liquidityFees: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub blockRewards: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub earnings: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub bondingEarnings: u128,
    #[serde(deserialize_with = "string_to_u128")]
    pub liquidityEarnings: u128,
    #[serde(deserialize_with = "string_to_f64")]
    pub avgNodeCount: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub runePriceUSD: f64,
    pub pools: Vec<PoolData>,
}

// Top-level struct
#[derive(Deserialize, Debug)]
pub struct RootEarnDetails {
    pub meta: RunePoolMeta,
    pub intervals: Vec<RunePoolInterval>,
}
