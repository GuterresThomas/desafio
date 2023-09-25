pub mod http_server {
    use warp::Filter;


    #[tokio::main]
    pub async fn main() {
        // Defina um filtro simples que corresponda a todas as solicitações HTTP na raiz ("/")
        
        let hello = warp::path("hello")
            .map(|| warp::reply::html("Hello, World!"));
    
        // Inicie o servidor na porta 8080
        warp::serve(hello)
            .run(([127, 0, 0, 1], 8080))
            .await;
    }
}