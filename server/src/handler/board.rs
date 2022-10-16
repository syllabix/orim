use std::time::Instant;

use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web, Error, HttpRequest, HttpResponse, ResponseError,
};
use actix_web_actors::ws;
use actix_session:: Session;
use derive_more::{Display, Error};

use crate::{
    board::{self, space, user::User, Registry},
    user,
};

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "You are not authorized to access this board")]
    UserNotAuthorized,

    #[display(fmt = "The board id is invalid")]
    InvalidBoardId,
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::UserNotAuthorized => StatusCode::UNAUTHORIZED,
            ServerError::InvalidBoardId => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

fn get_user(
    _req: &HttpRequest,
    registry: web::Data<user::Registry>,
    session: Session,
) -> Result<user::Participant, Error> {
    if let Some(user_id) = session.get::<String>("token")? {
        match registry.get(user_id.parse::<u16>().unwrap()) {
            Some(user) => Ok(user),
            None => Err(Error::from(ServerError::UserNotAuthorized)),
        }
    } else {
        return Err(Error::from(ServerError::UserNotAuthorized));
    }
}

fn get_space_id(req: &HttpRequest) -> Result<space::ID, Error> {
    let space_id: &str = match req.match_info().get("id") {
        Some(id) => id,
        None => return Err(Error::from(ServerError::InvalidBoardId)),
    };

    match space_id.trim().parse() {
        Ok(id) => Ok(id),
        Err(_) => Err(Error::from(ServerError::InvalidBoardId)),
    }
}

pub async fn connect(
    req: HttpRequest,
    stream: web::Payload,
    spaces: web::Data<board::Registry>,
    users: web::Data<user::Registry>,
    session: Session
) -> Result<HttpResponse, Error> {
    println!("Connect entry point");
    let user = get_user(&req, users, session)?;
    let space_id = get_space_id(&req)?;
    let space = spaces.get_or_create(space_id);

    println!("Start WS connection!");
    ws::start(
        User {
            user_id: user.id.into(),
            name: user.name,
            color: user.color,
            addr: space.clone(),
            heartbeat: Instant::now(),
        },
        &req,
        stream,
    )
}

pub async fn get_widgets(
    space_id: web::Path<space::ID>,
    server: web::Data<Registry>,
) -> HttpResponse {
    let space_id = space_id.into_inner();
    match server.get_space_info(space_id).await {
        Some(info) => HttpResponse::Ok().json(info),
        None => HttpResponse::NotFound().finish(),
    }
}
