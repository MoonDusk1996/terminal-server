use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use clipboard::{ClipboardContext, ClipboardProvider};
use notify_rust::Notification;
use serde::Deserialize;

#[derive(Deserialize)]
struct TextData {
    text: String,
}
#[post("/set_clipboard")]
async fn set_clipboard(data: web::Json<TextData>) -> impl Responder {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    match ctx.set_contents(data.text.clone()) {
        Ok(_) => {
            Notification::new()
                .summary("Numero de serial copiado para área de transferência")
                .body(&data.text.clone())
                .show()
                .unwrap();
            HttpResponse::Ok().body("Texto copiado para a área de transferência!")
        }
        Err(_) => HttpResponse::InternalServerError()
            .body("Erro ao copiar texto para a área de transferência."),
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(set_clipboard))
        .bind("0.0.0.0:30001")?
        .run()
        .await
}
