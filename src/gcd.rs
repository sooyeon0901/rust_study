// use std::fmt::format;
use gcd::Gcd;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize};

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
                .route("/", web::get().to(get_index))
                .route("/gcd", web::post().to(post_gcd))
    });

    println!("serving on http://localhost:3000...");
    server.bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
                .content_type("text/html")
                .body(r#"
                        <title>GCD Calculator</title>
                        <form action="/gcd" method="post">
                            <input type="text" name="n"/>
                            <input type="text" name="m"/>
                            <button type="submit">Compute GCD</button>
                        </form>
                    "#,
                )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
                            .content_type("text/html")
                            .body("Computng the GCD with zero is boring");
    }

    let response = 
        format!("The greatest common divisor if the numbers {} and {} is <b>{}</b>\n",
                form.n, form.m, form.n.gcd(form.m));
    
    HttpResponse::Ok().content_type("text/html").body(response)
}

