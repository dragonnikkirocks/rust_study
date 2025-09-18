use reqwest;
use tokio;
// Bring Tokio runtime into the scope.
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url= "https://www.rust-lang.org";
    // Asynchronous GET request.
    // let response = reqwest::get(url).await?;
    // // Extract and print the response text.
    // let response_body = response.text().await?;

    let response_body = reqwest::get(url).await?.text().await?; //multiple await - as each step depends on output of last step

    //The ? operator is Rust’s error propagation shortcut.
//If the Result is Ok(resp), it unwraps it and assigns it to resp.
//If the Result is Err(e), it immediately returns that error from the current function.
// manually can be written with match steps
//
// let resp = match client.get(url).await {
//     Ok(r) => r,            // unwrap the value
//     Err(e) => return Err(e), // early return from this function
// };
//
    println!("Response Body: {:?}", response_body);
    Ok(())
}

async fn bad_example(url1: &str, url2: &str) -> Result<(String,String), reqwest::Error>
{
    let response1 = reqwest::get(url1).await?.text().await?;
    let response2 = reqwest::get(url2).await?.text().await?;
    Ok((response1,response2))
}

async fn good_example(url1: &str, url2: &str) -> Result<(String,String), reqwest::Error>
{
 //create future
 //here future is just created and not run - super important point!!!!!
 // future in Rust is lazy - doesnt run until run qith a runtime such as tokio
 // why rust does it like this ?
 //Keeps things deterministic: nothing runs behind your back.
//Lets you control when work begins — super important for performance.
//Makes it possible to build efficient combinators like join!, select!, and join_all.
       let response1_future = reqwest::get(url1);
    let response2_future = reqwest::get(url2);

      //join is basically batched await 
    let (response1,response2) = tokio::join!(response1_future,response2_future);

    // Now await text bodies (can also do in parallel)
    let (body1, body2) = tokio::join!(response1?.text(), response2?.text());

    Ok((body1?, body2?))

}