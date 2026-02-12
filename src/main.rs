use axum::{
    response::Html,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // 1. Define your routes
    let app = Router::new()
        .route("/", get(handler));

    // 2. Define the address (localhost:3000)
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("--> Server running at http://{}", addr);

    // 3. Start the server
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Rust Local UI</title>
                <style>
                    body { font-family: sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; background: #1a1a1a; color: white; }
                    .card { border: 2px solid #f74c00; padding: 2rem; border-radius: 10px; text-align: center; }
                    button { background: #f74c00; border: none; padding: 10px 20px; color: white; border-radius: 5px; cursor: pointer; }
                </style>
            </head>
            <body>
                <div class="card">
                    <h1>Rust + Browser UI</h1>
                    <p>The backend is powered by 🦀 Rust.</p>
                    <button onclick="alert('You clicked the Rust-served button!')">Click Me</button>
                </div>
            </body>
        </html>
    "#)
}
