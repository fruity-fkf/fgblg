use axum::{routing::get_service, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use std::io;

pub async fn serve_web(dir: &str, port: u16) -> io::Result<()> {
    // Check if directory exists
    if !std::path::Path::new(dir).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory '{}' does not exist", dir),
        ));
    }

    let service = get_service(ServeDir::new(dir));

    let app = Router::new().nest_service("/", service);
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Server started. The '{}' folder is being served at http://{}", dir, address);

    let listener = tokio::net::TcpListener::bind(address).await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to bind to address: {}", e)))?;

    axum::serve(listener, app).await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Server error: {}", e)))?;

    Ok(())
}
