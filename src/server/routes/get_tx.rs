use actix_web::{HttpResponse, web::Data};

use crate::{server::error::ValidatorServerError, database::{schema::transactions::dsl::*, models::Transaction}, types::DbPool};
use diesel::prelude::*;

pub async fn get_tx(path: (String,), db: Data<DbPool>) -> actix_web::Result<HttpResponse, ValidatorServerError> {
    let res = actix_rt::task::spawn_blocking(move || {
        let conn = db.get().unwrap();
        transactions
            .filter(id.eq(path.0))
            .first::<Transaction>(&conn)
    })
        .await?;

    if let Ok(r) = res {
        Ok(HttpResponse::Ok().json(r))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}