use actix_web::web;

use crate::api::chat::chat_controllers;
use crate::api::user::user_controllers;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .configure(user_controllers::routes)
            .configure(chat_controllers::routes),
    );
}
