use aleph_client::{Connection, KeyPair, SignedConnection};

pub type WsConnection = Connection;
pub type SignedWsConnection = SignedConnection;

pub async fn init(url: &str) -> WsConnection {
    Connection::new(url).await
}

pub fn signed_connection(connection: &WsConnection, keypair: &KeyPair) -> SignedWsConnection {
    let signer = KeyPair::new(keypair.signer().clone());
    SignedWsConnection::from_connection(connection.clone(), signer)
}
