use warp::Filter;
use tokio_postgres::{NoTls, Error};

mod tcp_server;
mod database;
mod http_server;
mod routes;




#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    use tcp_server::tcp_server::start_tcp_server;
    let tcp_server_result = tokio::spawn(start_tcp_server());
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
    let routes = warp::any().map(|| "Hello, World!");

    // Start the Warp HTTP server
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;

    

    
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

    use database::database::read_user;

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

    let _ = tcp_server_result.await;

    use warp::Filter;
   
    
    Ok(())
   
   
}




