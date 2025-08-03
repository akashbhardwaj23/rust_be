use poem::{handler, web::Json};

use crate::types::{request_output::{CreateUserOutput, SignInOutput}, requests_input::CreateUserInput};

use crate::middlleware::{auth::Token};



#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, _token : Token) -> Json<SignInOutput> {

    // println!("The value of token is {:?}", token);
    // change these not clone here
    let id = data.username + "shjdb";

    let jwt = id;

    let sign_in_output = SignInOutput{
        jwt
    };

    Json(sign_in_output)
}

#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput>{
    let id = data.username + "dbgd";



    let sign_up_output = CreateUserOutput{
        id
    };

    Json(sign_up_output)
}