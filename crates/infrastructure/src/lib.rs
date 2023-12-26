use std::env;
use std::net::TcpListener;

use actix_web::{App, dev::Server, HttpServer, web};

use adapters::api::shared::app_state::AppState;
use adapters::spi::user::user_repository::UserRepository;
use db::db_connection::DbConnection;

pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    let db_connection = DbConnection::new(db_name.to_string());

    let data = web::Data::new(AppState {
        app_name: String::from("Animal Facts API"),
        user_repository: UserRepository {
            db_connection
        },
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move ||
        App::new().app_data(data.clone()).configure(adapters::api::shared::routes::routes))
        .listen(listener)?
        .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}