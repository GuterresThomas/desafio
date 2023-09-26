pub mod http_server {
    use warp::Filter;


    #[tokio::main]
    pub async fn start_http_server() {
        // Define Warp routes
        let routes = warp::any().map(|| "Hello, World!");
    
        // Start the Warp HTTP server
        warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
}
