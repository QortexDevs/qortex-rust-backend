
mod config;
mod redis;

use crate::config::get_settings;
use actix_web::{get, App, HttpServer, HttpRequest, Responder, web};
use redis::{get_value, set_value, with_connection};


#[get("/hello")]
async fn hello() -> impl Responder {
    web::Json(serde_json::json!({ "message": "Hello from Actix Web" }))
}

#[get("/all-cookies")]
async fn all_cookies(req: HttpRequest) -> impl Responder {
    // Получаем все куки из запроса
    let cookies = match req.cookies() {
        Ok(cookies) => cookies,
        Err(_) => {
            return "Ошибка при чтении кук".to_string();
        }
    };
    
    // Формируем список кук в виде строки
    let cookies_list = cookies.iter()
        .map(|cookie| format!("{} = {}", cookie.name(), cookie.value()))
        .collect::<Vec<_>>()
        .join("\n");
    
    format!("Все куки:\n{}", cookies_list)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_settings();
    let rv = get_value("key");
    match rv {
        Ok(Some(value)) => print!("value: {}", value),
        Ok(None) => println!("Key not found (nil)"), 
        Err(e) => eprintln!("Error getting value: {}", e),
    }
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(all_cookies)
    })
    .bind(format!("{}:{}", settings.api_host, settings.api_port))?
    .run()
    .await
}