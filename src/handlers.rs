use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{UserCreateRequest,UserUpdateRequest,User};

pub async fn create_user(
    db: web::Data<PgPool>,
    req: web::Json<UserCreateRequest>,
) -> impl Responder {
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, first_name, last_name, email)
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
    let result = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY id")
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

pub async fn update_user(
    db: web::Data<PgPool>,
    path: web::Path<i32>,
    req: web::Json<UserUpdateRequest>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query_as::<_, User>(
        "UPDATE users SET
            username = COALESCE($1, username),
            first_name = COALESCE($2, first_name),
            last_name = COALESCE($3, last_name),
            email = COALESCE($4, email)
         WHERE id = $5
         RETURNING *"
    )
    .bind(&req.username)
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.email)
    .bind(user_id)
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().body("Error updating user")
        }
    }
}

pub async fn delete_user(
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().body("Error deleting user")
        }
    }
}
