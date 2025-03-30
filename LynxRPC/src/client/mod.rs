use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use crate::core::{RpcCodec, Error};
use crate::core::codec::{serialize, deserialize};
use rand::Rng;
use parking_lot::Mutex;

pub struct Client {
    framed: Framed<TcpStream, RpcCodec>,
    pending_requests: Mutex<HashMap<u32, tokio::sync::oneshot::Sender<Vec<u8>>>>,
}

impl Client {
    pub async fn connect(addr: &str) -> Result<Self, Error> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self {
            framed: Framed::new(stream, RpcCodec),
            pending_requests: Mutex::new(HashMap::new()),
        })
    }

    pub async fn call<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &mut self,
        method: &str,
        auth_token: &str,
        params: T,
    ) -> Result<R, Error> {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen();
        let (tx, rx) = tokio::sync::oneshot::channel();

        self.pending_requests.lock().insert(id, tx);

        let request = serialize(&(
            method,
            auth_token,
            serialize(&params)?,
        ))?;

        self.framed.send((id, request)).await?;
        
        while let Some(Ok((response_id, data))) = self.framed.next().await {
            if response_id == id {
                return Ok(deserialize(&data)?);
            }
            if let Some(tx) = self.pending_requests.lock().remove(&response_id) {
                let _ = tx.send(data.to_vec());
            }
        }

        Err(Error::InvalidParams)
    }
}