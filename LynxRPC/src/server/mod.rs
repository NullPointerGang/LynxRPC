use std::{collections::HashMap, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;
use futures::{SinkExt, StreamExt};
use parking_lot::Mutex;
use crate::core::{RpcCodec, Error};
use crate::core::codec::{serialize, deserialize};
use crate::security::AuthValidator;

type Handler = Box<dyn Fn(&[u8]) -> Result<Vec<u8>, Error> + Send + Sync>;

pub struct Server {
    listener: TcpListener,
    handlers: Arc<Mutex<HashMap<String, Handler>>>,
    auth_validator: AuthValidator,
}

impl Server {
    pub async fn bind(addr: &str) -> Result<Self, Error> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self {
            listener,
            handlers: Arc::new(Mutex::new(HashMap::new())),
            auth_validator: AuthValidator::new(),
        })
    }

    pub fn register_handler<F, Req, Resp>(&self, method: &str, handler: F)
    where
        F: Fn(Req) -> Result<Resp, Error> + 'static + Send + Sync,
        Req: serde::de::DeserializeOwned,
        Resp: serde::Serialize,
    {
        let wrapped = move |data: &[u8]| -> Result<Vec<u8>, Error> {
            let req: Req = deserialize(data)?;
            let resp = handler(req)?;
            Ok(serialize(&resp)?)
        };

        self.handlers.lock().insert(method.to_string(), Box::new(wrapped));
    }

    pub async fn run(self) {
        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => {
                    let handlers = self.handlers.clone();
                    let validator = self.auth_validator.clone();
                    tokio::spawn(async move {
                        handle_connection(socket, handlers, validator).await;
                    });
                }
                Err(e) => eprintln!("Accept error: {}", e),
            }
        }
    }
}

async fn handle_connection(
    socket: TcpStream,
    handlers: Arc<Mutex<HashMap<String, Handler>>>,
    validator: AuthValidator,
) {
    let mut framed = Framed::new(socket, RpcCodec);

    while let Some(Ok((id, data))) = framed.next().await {
        let response = match process_request(&handlers, &validator, &data) {
            Ok(data) => data,
            Err(e) => serialize(&Response::Error(e.to_string())).unwrap(),
        };
        framed.send((id, response)).await.unwrap();
    }
}

fn process_request(
    handlers: &Arc<Mutex<HashMap<String, Handler>>>,
    validator: &AuthValidator,
    data: &[u8],
) -> Result<Vec<u8>, Error> {
    #[derive(serde::Deserialize)]
    struct Request {
        method: String,
        auth_token: String,
        params: Vec<u8>,
    }

    let req: Request = deserialize(data)?;
    
    if !validator.validate_token(&req.auth_token) {
        return Err(Error::AuthError);
    }

    let handlers = handlers.lock();
    let handler = handlers.get(&req.method)
        .ok_or(Error::MethodNotFound)?;

    handler(&req.params)
}

#[derive(serde::Serialize)]
enum Response {
    Success(Vec<u8>),
    Error(String),
}