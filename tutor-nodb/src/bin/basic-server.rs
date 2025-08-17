// Module imports
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    // For HTTP GET requests coming in on the /health route, the Actix
    // web server will route the request to health_check_handler().
    cfg.route("/health", web::get().to(health_check_handler));
}

//Configure handler
pub async fn health_check_handler() -> impl Responder {
    // The handler constructs an HTTP response with a greeting.
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

// Instantiate and run the HTTP server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Construct app and configure routes
    // Construct an Actix web application instance, and register the configured routes.
    let app = move || App::new().configure(general_routes);
    // Start HTTP server
    // Initialize a web server, load the application,
    // bind it to a socket, and run the server.
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
