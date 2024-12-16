use std::collections::HashMap;

pub fn creat_hashmap_basic() {
    // let mut stock_prices: HashMap<String, f32> = HashMap::new();
    let mut stock_prices = HashMap::<String, f32>::new();
    std::println!("{}", stock_prices.len());
    std::println!("{}", stock_prices.is_empty());

    stock_prices.insert("AAPL".to_string(), 123.45);
    stock_prices.insert("GOOGL".to_string(), 234.56);
    stock_prices.insert("AMZN".to_string(), 345.67);

    std::println!("{:#?}", stock_prices);

    stock_prices.insert("AAPL".to_string(), 423.45);
    std::println!("{:#?}", stock_prices);

    stock_prices.remove(&("GOOGL".to_string()));
    std::println!("{:#?}", stock_prices);

    // this is used in case when dev is not sure if the key is present or not, if present then wont update else will
    // to just update a value you use the insert method
    stock_prices.entry("META".to_string()).or_insert(528.33);
    std::println!("{:#?}", stock_prices);

    // used to iter through the hashmap
    for (ticker, value) in stock_prices.clone() {
        std::println!("{} : {}", ticker, value);
    }
}
