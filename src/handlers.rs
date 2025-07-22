use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{UserCreateRequest, User};

pub async fn create_user(
    db: web::Data<PgPool>,
    req: web::Json<UserCreateRequest>,
) -> impl Responder {
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO testing.users (username, first_name, last_name, email)
         VALUES ($1, $2, $3, $4)
         RETURNING *"
    )
    .bind(&req.username)
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.email)
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().body("Error creating user")
        }
    }
}

pub async fn get_users(db: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>("SELECT * FROM testing.users ORDER BY id")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().body("Error fetching users")
        }
    }
}
