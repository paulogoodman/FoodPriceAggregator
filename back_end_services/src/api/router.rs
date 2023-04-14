use actix_web::{web, App, HttpServer};
use handler::init_routes; // Import the `init_routes` function from the handler module.

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the Tokio runtime with the default number of worker threads.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        // Start the Actix web server.
        HttpServer::new(|| {
            let app = App::new();
            init_routes(&mut app.service(web::scope("/api")));
            app
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    })
}
