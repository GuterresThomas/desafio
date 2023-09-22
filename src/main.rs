use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use tokio_postgres::{Client, GenericClient};


#[tokio::main]
async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

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

use tokio_postgres::{NoTls, Error};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    /* let user_document = "exemplo";
    let credit_card_token = "token123";
    let value = 100;

    match add_user(&client, user_document, credit_card_token, value).await {
        Ok(()) => {
            println!("Usuário adicionado com sucesso.");
        }
        Err(e) => {
            eprintln!("Erro ao adicionar o usuário: {}", e);
        }
    } */

    match read_user(&client).await {
        Ok(users) => {
            println!("Usuários lidos com sucesso:");
            for user in users {
                println!("{:?}", user);
            }
        }
        Err(e) => {
            eprintln!("Erro ao ler os usuários: {}", e);
        }
    }

    /*let user_id_to_update = 1;
    let new_value = 200;

    match update_user(&client, user_id_to_update, new_value).await {
        Ok(()) => {
            println!("Usuário atualizado com sucesso.");
        }
        Err(e) => {
            eprintln!("Erro ao atualizar o usuário: {}", e);
        }
    }*/

    /* let user_id_to_delete = 1;

    match delete_user(&client, user_id_to_delete).await {
        Ok(()) => {
        println!("Usuário excluido com sucesso!");
        }
        Err(e) => {
            eprintln!("Erro ao excluir o usuário: {}", e);
        }
    } */


    Ok(())
}


