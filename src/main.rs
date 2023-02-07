// Actix Web    - API       
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder}; 

#[get("/")]  
async fn hello() -> impl Responder { 
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body) 
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
} 
 Main Function   
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}  

// MongoDB Connection 

use mongodb::{ 
    bson::doc,
    sync::Client, 
};

use serde::{Deserialize, Serialize}; 

#[derive(Debug, Serialize, Deserialize)]  
struct User{ 
    // field: Type
    name: String, 
    password: String,

}

fn add_user(client: mongodb::sync::Client){   
    let db = client.database("test"); 
    let coll = db.collection("users"); 
    coll.insert_one(doc!("name" : "suman", "password": "crypto"), None); 
}

fn find_user(coll: mongodb::sync::Collection<User>){  
    let cursor = coll.find(doc! {"name": "saurabh"}, None).unwrap();  
    for result in cursor {
        println!("{:?}", result)  
    }
}

fn main(){
    println!("Hello DB User");  
    let client = Client::with_uri_str("mongodb+srv://suman:12345@cluster0.hrqi2li.mongodb.net/?retryWrites=true&w=majority").unwrap(); 
    // add_user(client);   
    let db = client.database("test");
    let coll = db.collection::<User>("users");  
    find_user(coll); 
}  

