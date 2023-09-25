pub mod database {
    use tokio_postgres::{Client, Error};
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Serialize, Deserialize)]

    
    pub struct User {
        id: i32,
        user_document: String,
        credit_card_token: String,
        value: i32,
    }

    impl User {
        fn build_user(id: i32, user_document: String, credit_card_token: String, value: i32) -> User {
            User {
                id,
                user_document,
                credit_card_token,
                value,
            }
        }
    }
    

pub async fn add_user(
    client: &tokio_postgres::Client,
    user_document: &str,
    credit_card_token: &str,
    value: i32,
)-> Result<(), Error>{
    let query = format!(
        "INSERT INTO users_rust (
        user_document, credit_card_token, value)
        VALUES('{}', '{}', '{}')",
        user_document, credit_card_token, value
    );
    client.execute(&query, &[]).await?;

    Ok(())
}


pub async fn read_user(client: &Client) -> Result<Vec<User>, Error> {
    let query = "SELECT id, user_document, credit_card_token, value FROM users_rust";
    let rows = client.query(query, &[]).await?;
    

    let mut users = Vec::new();

    for row in rows {
        let id:i32 = row.get(0);
        let user_document:String = row.get(1);
        let credit_card_token:String = row.get(2);
        let value:i32 = row.get(3);

        let user = User::build_user(id, user_document, credit_card_token, value);
        users.push(user)
    }

    Ok(users)
}

pub async fn update_user(
    client: &Client,
    id: i32,
    new_value: i32,
) -> Result<(), Error> {
    let query = format!("UPDATE users_rust SET value = {} WHERE id = {}",
    new_value, id);
    client.execute(&query, &[]).await?;

    Ok(())
}

pub async fn delete_user(client: &Client, user_id:i32) -> Result<(), Error> {   
    let query = format!("DELETE FROM users_rust WHERE id = {}", user_id);
    client.execute(&query, &[]).await?;

    Ok(())
}





}