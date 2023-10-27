// use std::env;
// use std::str::FromStr;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct SumParameters{
    n: u64,
    m: u64,
}

fn main() {

    let server = HttpServer::new(||{
        App::new()
                .route("/", web::get().to(get_index))
                .route("/sum", web::post().to(post_sum))
    });

    println!("Serving on http://localhost:3000");
    server.bind("127.0.0.1:3000").expect("Error binding server to address.").run().expect("error running server.");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title> Sum Calculator </title>
            <form action="/sum" method="post">
            <input type="text" name="n" />
            <input type="text" name="m" />
            <button type="submit"> Compute SUM </button>
            </form>
        "#,
    )
}

fn post_sum(form: web::Form<SumParameters>) -> HttpResponse{
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("Computing the sum  with zero is boring.");
    }

    let response = format!("The sum of the numbers {} and {} is <b>{}<b>\n",form.n,form.m,form.n+form.m);

    HttpResponse::Ok().content_type("text/html").body(response)
}