mod api;
    mod models;
    mod repository;

    //modify imports below
    use actix_web::{web::Data, App, HttpServer};
    use api::user_api::{hello,create_user, delete_user, get_all_users, get_user, update_user};
    use repository::mongodb_repo::MongoRepo;

    use crate::api::user_api::{get_user_using_email, login_user};
    
    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        let db = MongoRepo::init().await;
        let db_data = Data::new(db);

        println!("The is running on http://localhost:8080");

        HttpServer::new(move || {
            App::new()
                .app_data(db_data.clone())
                .service(hello)
                .service(create_user)
                .service(get_user)
                .service(get_user_using_email)
                .service(update_user)
                .service(delete_user)
                .service(get_all_users)
                .service(login_user)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
