use actix_web::web;

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/test").route("/foo", web::get().to(foo)));
}

async fn foo() -> String {
    "foo".to_string()
}
