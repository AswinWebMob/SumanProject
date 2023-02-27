mod api;   
mod models;
mod repository;
mod jwt_sign;


use actix_web::{web, web::Data ,App, HttpServer};  
use api::user_api::{add_marks};
    use mongodb::Client;
    //  update_marks,get_result, delete_marks, get_all_students, create_user, delete_user, get_all_users, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

pub struct TempClientConn {
    client_conn: MongoRepo, 
}

impl TempClientConn {
    pub fn new(client_conn: MongoRepo) -> Self { 
        TempClientConn { client_conn }
    }
}

pub struct AppState {
    client_conn: mongodb::Client,  
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let db = MongoRepo::init().await;
    // let db_data = Data::new(db);

    let uri = "w+srv://webmob:webmob@cluster0.gdvte43.mongodb.net";   

    let mongoDB_url =  mongodb::options::ClientOptions::parse(uri).await.unwrap_or_default();
    let client_conn = mongodb::Client::with_options(mongoDB_url).unwrap(); 

    // let data_app = ;


    HttpServer::new(move || { 
        App::new() 
            .app_data(AppState { client_conn })          // FnOnce()   
            .route("/teacher/addmarks/",web::post().to(add_marks)) 
            // .route("/student/{id}",web::get().to(get_result))
            // .route("/teacher/editmarks/{id}",web::put().to(update_marks)) 
            // .route("/teacher/deletemarks/{id}",web::delete().to(delete_marks))    
            // .route("/teacher/getallstudents/{id}",web::get().to(get_all_students))  


            // .service(create_user)  
            // .service(get_user)
            // .service(update_user)
            // .service(delete_user)
            // .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
