pub mod server;
pub mod client;

use aul::level::Level;
use wbsl::error::WBSLError;
use wbsl::methods::Methods::{Get, Post};
use wbsl::server::Server;
use whdp::{Request, Response};
use whdp::resp_presets::{bad_request, created, ok};
use wjp::{map, ParseError, Serialize, SerializeHelper, Values};

#[derive(Debug)]
struct Idk {
    message: String,
    code: u16,
}

impl Serialize for Idk {
    fn serialize(&self) -> Values {
        Values::Struct(map!(("message",self.message.serialize()),("code",self.code.serialize())))
    }
}

impl TryFrom<Values> for Idk {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let message = struc.get_val_opt("message", |val| val.get_string()).ok_or(ParseError::new())?;
        let code = struc.map_val("code", u16::try_from)?;
        Ok(Self {
            code,
            message,
        })
    }
}

static mut MAP: Vec<Idk> = Vec::new();

fn main() -> Result<(), WBSLError> {
    Server::builder()
        .with_auto_headers("pet-store", "application/json")
        .with_logging(Level::INFO)
        .route("/", Get(unsafe_get))
        .route("/", Post(unsafe_insert))
        .listen("0.0.0.0:6969")?
        .start();
    Ok(())
}

pub fn unsafe_get(_req: Request) -> Response {
    unsafe { get_all() }
}

pub fn unsafe_insert(req: Request) -> Response {
    unsafe { insert(req) }
}

unsafe fn get_all() -> Response {
    println!("{:?}", MAP);
    ok(MAP.json())
}

unsafe fn insert(req: Request) -> Response {
    let res = req.get_parsed_body();
    if res.is_ok() {
        let res = res.unwrap();
        println!("{:?}", res);
        MAP.push(res);
        println!("{:?}", MAP);
        created(String::new())
    } else {
        bad_request(String::new())
    }
}