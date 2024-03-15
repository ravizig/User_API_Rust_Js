use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Responder,
};
use mongodb::bson::oid::ObjectId;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[post("/user/create")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let email = new_user.email.to_owned();
    if email.is_empty() {
        return HttpResponse::BadRequest().body("Invalid Email");
    } else {
        let user_detail = db.get_user_using_email(&email).await;
        if !user_detail.is_ok() {
            let data = User {
                id: None,
                username: new_user.username.to_owned(),
                email: new_user.email.to_owned(),
                password: new_user.password.to_owned(),
            };
            let user_detail = db.create_user(data).await;
            match user_detail {
                Ok(user) => return HttpResponse::Ok().json(user),
                Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        return HttpResponse::Ok().body("User already exists");
    }
}

// #[post("/user/login")]
// pub async fn login_user(db: Data<MongoRepo>, login_data: Json<User>) -> HttpResponse {
//     println!("{:?}", login_data);
//     let email = login_data.email.to_string();
//     let provided_password = login_data.password.to_string();

//     let user_detail = db.get_user_using_email(&email).await;

//     let stored_password = user_detail.as_ref().unwrap().password.clone();

//     println!("{}", &stored_password);

//     match bcrypt::verify(&provided_password, &stored_password) {

//         Ok(valid) => {
//             if valid {
//                 return HttpResponse::Ok().json("Login successful");
//             } else {
//                 return HttpResponse::Unauthorized().body("Invalid credentials");
//             }
//         }
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
//     // return HttpResponse::Ok().json("Login successful");
// }

#[post("/user/login")]
pub async fn login_user(db: web::Data<MongoRepo>, login_data: web::Json<User>) -> HttpResponse {
    println!("{:?}", login_data);
    let email = login_data.email.to_string();
    let provided_password = login_data.password.to_string();

    let user_detail = db.get_user_using_email(&email).await;

    let stored_password = user_detail.as_ref().unwrap().password.clone();

    match bcrypt::verify(&provided_password, &stored_password) {

        Ok(valid) => {
            if valid {
                return HttpResponse::Ok().json("Login successful");
            } else {
                return HttpResponse::Unauthorized().body("Invalid Password");
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/user/get/email/{email}")]
pub async fn get_user_using_email(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let email = path.into_inner();
    if email.is_empty() {
        return HttpResponse::BadRequest().body("invalid Email");
    }
    let user_detail = db.get_user_using_email(&email).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/get/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/user/update/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let update_result = db.update_user(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with this id");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user/delete/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("User with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/getall")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
