use actix_files::{Files};
use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde_json::json;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Neighbourhood Grocery Market",
        "project_short_name": "NGM",
        "categories":["Fruit","Vegetable","Pasta","Dessert"],
        "items":[
            {
                "name":"Apple",
                "category":"Fruit",
                "price": 3,
                "image_path":"/static/images/Apple.png"
            },
            {
                "name":"Banana",
                "category":"Fruit",
                "price": 2,
                "image_path": "/static/images/Banana.png"
            },
            {
                "name":"Peach",
                "category":"Fruit",
                "price": 3,
                "image_path":"/static/images/Peach.png"
            },
            {
                "name":"Broccoli",
                "category":"Vegetable",
                "price": 3,
                "image_path":"/static/images/Broccoli.png"
            },
            {
                "name":"Kale",
                "category":"Vegetable",
                "price": 5,
                "image_path":"/static/images/Kale.png"
            },
            {
                "name":"Chicken alfredo",
                "category":"Pasta",
                "price": 7,
                "image_path":"/static/images/Chicken alfredo.png"
            },
            {
                "name":"Pie",
                "category":"Dessert",
                "price": 4,
                "image_path":"/static/images/Pie.png"
            }
        ]


    });

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)

    // Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
