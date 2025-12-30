use actix_web::{HttpResponse, Result};
use tera::{Tera, Context};

/// Teraテンプレートエンジンの初期化
pub fn init_tera() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    }
}

/// テンプレートをレンダリングしてHttpResponseを返す
pub fn render_template(tera: &Tera, template_name: &str, context: Context) -> Result<HttpResponse> {
    match tera.render(template_name, &context) {
        Ok(body) => Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(body)),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            Ok(HttpResponse::InternalServerError()
                .body(format!("Template error: {}", e)))
        }
    }
}

