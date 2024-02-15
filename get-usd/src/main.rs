use common::exchance_service::convert_brl_to_usd;
use bigdecimal::BigDecimal;
use lambda_http::{run, service_fn, Body, Error, Request, Response, RequestExt};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(move |event: Request| async move {
        get_usd(event).await
    })).await
}

async fn get_usd(event: Request) -> Result<Response<Body>, Error> {
    let map = event.query_string_parameters();
    let amount_in_brl = map.first("amount").unwrap_or_default();
    println!("process=get_usd, status=started, amount_in_brl='{}'", amount_in_brl);
    let amount_in_brl_dec = BigDecimal::from_str(amount_in_brl)?;

    let result = convert_brl_to_usd(&amount_in_brl_dec).await?;

    println!("process=get_usd, status=success, card_id='{}'", result);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result.to_string().into())
        .map_err(Error::from)?)
}