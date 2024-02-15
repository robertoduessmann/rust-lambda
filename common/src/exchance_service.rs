use bigdecimal::BigDecimal;
use lambda_http::Error;

pub async fn convert_brl_to_usd(
    amount_in_brl: &BigDecimal
) -> Result<BigDecimal, Error> {
    return Ok(amount_in_brl * 5); 
}
