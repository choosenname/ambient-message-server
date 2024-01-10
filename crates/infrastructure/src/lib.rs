use std::env;
use std::net::TcpListener;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, dev::Server, HttpServer, web};

use adapters::api::shared::app_state::AppState;
use adapters::services::crypto::CryptoService;
use adapters::spi::chat::chat_repository::ChatRepository;
use adapters::spi::user::user_repository::UserRepository;
use adapters::spi::user_chat::user_chat_repository::UserChatRepository;
use db::db_connection::DbConnection;

pub fn server(listener: TcpListener, app_name: &str) -> Result<Server, std::io::Error> {
    //let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    let db_connection = Arc::new(DbConnection::new(database_url));

    let data = web::Data::new(AppState {
        app_name: String::from(app_name),
        user_repository: UserRepository {
            db_connection: db_connection.clone(),
        },
        chat_repository: ChatRepository {
            db_connection: db_connection.clone(),
        },
        user_chat_repository: UserChatRepository {
            db_connection: db_connection.clone(),
        },
        crypto_services: CryptoService {
            //todo get from env
            jwt_secret: String::from("Bebra").into(),
        }
    });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type"]),
            )
            .app_data(data.clone())
            .configure(adapters::api::shared::routes::routes)
    })
        .listen(listener)?
        .run();

    Ok(server)
}
