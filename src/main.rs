use warp::Filter;
use tokio_postgres::{NoTls, Error};
use serde::{Serialize, Deserialize};
mod tcp_server;
mod database;
mod http_server;
mod routes;
use std::sync::{Arc, Mutex};
use tokio::task;


// Importe o módulo database para usar as funções e tipos relevantes.
use database::database::{add_user, read_user, update_user, delete_user, User};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}




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

    
    let _ = tcp_server_result.await;


    let client = Arc::new(Mutex::new(client));

    // Filtro para fornecer o cliente do banco de dados a cada rota.
    let db_filter = warp::any().map(move || client.clone());

    // Rota para criar um novo usuário.
    let create_user = warp::path!("users" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_filter.clone()) // Use a captura de movimento para clonar o cliente.
        .and_then(|user: User, client: Arc<Mutex<tokio_postgres::Client>>| async move {
            let client = client.lock().unwrap(); // Desbloqueie o cliente dentro do Mutex.
            match add_user(&client, &user.user_document, &user.credit_card_token, user.value).await {
                Ok(()) => Ok(warp::reply::json(&user)),
                Err(e) => {
                    eprintln!("Erro ao criar usuário: {}", e);
                    Ok(warp::reply::json(&ErrorResponse { error: "Erro ao criar usuário".to_string() }))
                }
            }
        });

    // Rota para ler todos os usuários.
        let read_users = warp::path!("users" / "read")
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(|client: Arc<Mutex<tokio_postgres::Client>>| async move {
            let client = client.lock().unwrap();
            match read_user(&client).await {
                Ok(users) => Ok(warp::reply::json(&users)),
                Err(e) => {
                    eprintln!("Erro ao ler usuários: {}", e);
                    Ok(warp::reply::json(&ErrorResponse { error: "Erro ao ler usuários".to_string() }))
                }
            }
        });

        // Rota para atualizar um usuário.
        let update_user_route = warp::path!("users" / "update" / i32 / i32)
        .and(warp::put())
        .and(db_filter.clone())
        .and_then(|user_id: i32, new_value: i32, client: Arc<Mutex<tokio_postgres::Client>>| async move {
            let client = client.lock().unwrap();
            match update_user(&client, user_id, new_value).await {
                Ok(()) => Ok(warp::reply::json(&format!("Usuário {} atualizado com sucesso", user_id))),
                Err(e) => {
                    eprintln!("Erro ao atualizar usuário: {}", e);
                    Ok(warp::reply::json(&ErrorResponse { error: "Erro ao atualizar usuário".to_string() }))
                }
            }
        });

/*   // Rota para excluir um usuário.
let delete_user_route = warp::path!("users" / "delete" / i32)
.and(warp::delete())
.and(db_filter.clone())
.and_then(|user_id: i32, client: Arc<Mutex<tokio_postgres::Client>>| {
    let client_clone = client.clone();
    let task = async move {
        let client = client_clone.lock().unwrap();
        match delete_user(&client, user_id).await {
            Ok(_) => Ok(warp::reply::json(&format!("Usuário {} excluído com sucesso", user_id))),
            Err(e) => {
                eprintln!("Erro ao excluir usuário: {}", e);
                Ok(warp::reply::json(&ErrorResponse { error: "Erro ao excluir usuário".to_string() }))
            }
        }
    };

    warp::reply::future(task)
});*/



// Combine todas as rotas em uma única rota.
let routes = create_user
    .or(read_users)
    .or(update_user_route);
    //.or(delete_user_route); // Certifique-se de incluir a rota de exclusão, se ela existir.

// Inicie o servidor HTTP e use .await.unwrap() para obter o resultado.
warp::serve(routes)
    .run(([127, 0, 0, 1], 3030))
    .await
    .expect("Falha ao iniciar o servidor HTTP");
  
    
    Ok(())

   
   
}




