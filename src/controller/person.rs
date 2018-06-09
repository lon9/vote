use actix_web::*;
use futures::future::Future;

use model::person::{PersonList};

use AppState;

pub fn person_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(PersonList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(person_list) => 
                    Ok(HttpResponse::Ok().json(person_list)),
                Err(_) => 
                    Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}
