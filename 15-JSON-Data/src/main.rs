use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

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
        std::println!("{}", y.ok().unwrap());
    }

    deserialze_data();
}

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
