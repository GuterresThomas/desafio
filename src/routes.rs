/* 
use warp::{Filter, Reply, Rejection};
use serde_json::json;
use tokio_postgres::Client;
use crate::database::database::{User, add_user};

// Your route code
pub fn add_user_route(
    client: Client,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("add_user")
        .and(warp::post())
        .and(warp::body::json::<User>())
        
        
}

// Your handler code
async fn handle_add_user(
    user_data: User, // Assuming you've deserialized the JSON request into User struct
    client: Client,
) -> Result<impl Reply, Rejection> {
    if let Err(err) = add_user(&client, &user_data.user_document, &user_data.credit_card_token, user_data.value).await {
        return Err(warp::reject::custom(err));
    }

    let response = json!({"message": "Usuário adicionado com sucesso."});

    Ok(warp::reply::json(&response))
}

fn with_client(client: Client) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}
*/