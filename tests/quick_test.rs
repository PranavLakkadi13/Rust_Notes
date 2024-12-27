use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let https_client = httpc_test::new_client("http://localhost:8080")
        .ok()
        .unwrap();

    // this is for a normal https get request and gets the whole data
    // https_client.do_get("/hello").await?.print().await?;

    // this is used to pass argumnets
    https_client
        .do_get("/hello?name=Pranav")
        .await?
        .print()
        .await?;

    https_client.do_get("/hello2/Pranav").await?.print().await?;

    // this is to test the minimal fallback code where the path that is not in routes is sent and the fallback
    // is the triggered and the the static is using the base of the path as the path to call 
    // https_client.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
