mod util;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use awc::http::StatusCode;
use clap::Parser;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use util::cache_system::CacheSystem;
use util::cl_parser::Cli;
use sysinfo::System;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    // Create the cache system with Mutex so it's thread-safe.
    static ref CACHE: Arc<Mutex<CacheSystem>> = Arc::new(Mutex::new(CacheSystem::new(18_000_000)));
}

struct AppState {
    origin: String,
}

// This function is used to fetch the content either from the cache or origin.
#[get("/{content_path}")]
async fn serve_content(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    // Extract the content string
    let content_path: String = req
        .match_info()
        .get("content_path")
        .unwrap()
        .parse()
        .unwrap();

    // Check if the content exists in the cache.
    if CACHE.lock().await.contains_key(&content_path) {
        dbg!("cache!!!");

        HttpResponse::Ok().body(CACHE.lock().await.get(&content_path))
    } else { // Fetch the content from the origin.
        let client = awc::Client::default();
        let response = client
            .get(format!("http://{}:8080/{}", state.origin, content_path)) // <- Create request builder
            .insert_header(("Accept-Encoding", "gzip"))
            .insert_header(("User-Agent", "Actix-web"))
            .send() // <- Send http request
            .await;

        match response {
            Ok(mut res) => match res.status() {
                StatusCode::OK => { // When the content was successfully fetched from the origin, serve it to the client.
                    let body_bytes = res.body().limit(20_000_000).await.expect("failed!!!");
                    let body = String::from_utf8_lossy(&body_bytes).to_string();
                    // Pass the content to the cache system and let it decides whether the content should be stored or not.
                    CACHE.lock().await.add(&content_path, &body);
                    HttpResponse::Ok().body(body)
                }
                _ => HttpResponse::NotFound().body(""),
            },
            Err(_) => HttpResponse::NotFound().body(""),
        }
    }
}

// This function is used to report the CPU usage of the HTTP server when the DNS server request it.
#[get("/api/getUsage")]
async fn get_usage() -> impl Responder {

    let mut sys = System::new();
    sys.refresh_cpu();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu();

    let mut usage = 0_f32;
    let mut cnt = 0_f32;
    for cpu in sys.cpus() {
        usage += cpu.cpu_usage();
        cnt += 1_f32;
    }

    // Calculate the average cpu usage
    usage = usage / cnt;

    HttpResponse::Ok().body(format!("{}", usage))
}

// This function is used to respond to the grading beacon.
#[get("/grading/beacon")]
async fn respond_beacon() -> impl Responder {
    HttpResponse::NoContent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // Used web::Data to pass the origin to each thread.
    let app_state = web::Data::new(AppState { origin: cli.origin });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(respond_beacon)
            .service(serve_content)
            .service(get_usage)
    })
    .keep_alive(Duration::from_secs(25))
    .bind(("0.0.0.0", cli.port))?
    .bind(("0.0.0.0", cli.port + 1))?
    .run()
    .await
}
