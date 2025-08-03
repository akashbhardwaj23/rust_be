use poem::{http::StatusCode, Error, FromRequest, Request, RequestBody, Result};


// Custom Token extractor
pub struct Token(String);

impl <'a> FromRequest<'a> for Token {
   async fn from_request(req : &Request,_body : &mut RequestBody) -> Result<Self>{
    let token = req
                        .headers()
                        .get("authorization")
                        .and_then(|value| value.to_str().ok())
                        .ok_or_else(|| Error::from_string("Error while Parsing the token", StatusCode::BAD_REQUEST))?;


    println!("The token is {}", token);
    Ok(Token(token.to_string()))
   }
}