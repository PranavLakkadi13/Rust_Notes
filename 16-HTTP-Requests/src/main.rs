use reqwest::blocking::{Client, ClientBuilder};
use reqwest::redirect::Policy;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    response: String,
}


// was using Mackoon for the api testing.....
fn main() {
    let https_client = Client::new();

    let https_result = https_client.get("http://trevorsullivan.net").send();

    if https_result.is_ok() {
        std::println!("{:#?}", https_result.ok().unwrap().text());
        std::println!("............................ executed ⬆️");
    } else {
        std::println!("{:#?}, is the error that is received", https_result.err());
    }

    let https_request_dev = https_client.get("http://localhost:3000/Hello").send();
    std::println!("{:#?}", https_request_dev.ok().unwrap().text());
    std::println!("............................ executed ⬆️");

    // this is a simple post request and has a timeout feature
    // let http_response_post = https_client
    //     .post("http://localhost:3000/Send_Data")
    //     .timeout(std::time::Duration::new(5, 0))
    //     .body("This is a post request!!!!")
    //     .send();

    // if http_response_post.is_ok() {
    //     std::println!("{:#?}", http_response_post.unwrap().text().unwrap());
    //     std::println!("............................ executed ⬆️");
    // }

    let new_http_response_post = https_client
        .post("http://localhost:3000/Send_Data")
        .body("This is the new post request")
        .header("User-Agent", "MY New Rust APP!!")
        .send();
    if new_http_response_post.is_ok() {
        let temp: String = new_http_response_post.unwrap().text().unwrap();

        let the_ptr_temp = temp.as_str();

        let read_json_data = from_str::<Response>(the_ptr_temp);

        if read_json_data.is_ok() {
            println!(
                "{:#?} has been deserialised!!!",
                read_json_data.ok().unwrap()
            );
        } else {
            std::println!("{:#?} is the error returned", read_json_data.err());
        }
    }

    // here we will see about redirects and how to deal with it
    let redir_policy = Policy::limited(5);
    let http_client_for_redirect = ClientBuilder::new()
        .redirect(redir_policy)
        .build()
        .ok()
        .unwrap();

    let the_result_returned = http_client_for_redirect
        .get("http://localhost:3000/Weather")
        .send();

    if the_result_returned.is_ok() {
        std::println!("{:?}", the_result_returned.ok().unwrap().text().unwrap())
    }
}
