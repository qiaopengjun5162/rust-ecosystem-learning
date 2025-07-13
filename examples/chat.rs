use std::{fmt, net::SocketAddr, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt, stream::SplitStream};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

const MAX_MESSAGES: usize = 128;

#[derive(Debug, Default)]
struct State {
    peers: DashMap<SocketAddr, mpsc::Sender<Arc<Message>>>,
}

#[derive(Debug)]
struct Peer {
    username: String,
    stream: SplitStream<Framed<TcpStream, LinesCodec>>,
}

#[derive(Debug)]
enum Message {
    UserJoined(String),
    UserLeft(String),
    Chat { sender: String, content: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    // console_subscriber::init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);

    let state = Arc::new(State::default());

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from {}", addr);
        let state_cloned = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(state_cloned, addr, stream).await {
                warn!("Failed to handle client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(state: Arc<State>, addr: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut stream = Framed::new(stream, LinesCodec::new());
    // 按帧发送的， LinesCodec 会在每行末尾加上 \n
    stream
        .send("Welcome to the chat! Please enter your username:")
        .await?;

    let username = match stream.next().await {
        Some(Ok(username)) => username,
        Some(Err(e)) => {
            warn!("Failed to receive username from {}: {}", addr, e);
            return Err(e.into());
        }
        None => {
            warn!("Client {} disconnected before sending username", addr);
            return Ok(());
        }
    };

    let mut peer = state.add(addr, username, stream).await;

    // notify others that a new peer has joined
    let message = Arc::new(Message::user_joined(&peer.username));
    info!("\x1b[32m🟢 用户加入: {:?}\x1b[0m", message);
    state.broadcast(addr, message).await;

    while let Some(line) = peer.stream.next().await {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                warn!("Failed to receive message from {}: {}", addr, err);
                break;
            }
        };

        let message = Arc::new(Message::chat(&peer.username, line));
        info!("\x1b[34m💬 聊天消息: {:?}\x1b[0m", message);
        state.broadcast(addr, message).await;
    }

    // when while loop exit, peer has left the chat or line reading failed
    // remove peer from state
    state.peers.remove(&addr);

    // notify others that peer has left the chat
    let message = Arc::new(Message::user_left(&peer.username));
    info!("\x1b[31m🔴 用户离开: {:?}\x1b[0m", message);
    state.broadcast(addr, message).await;

    Ok(())
}

impl State {
    async fn broadcast(&self, addr: SocketAddr, message: Arc<Message>) {
        for peer in self.peers.iter() {
            if peer.key() == &addr {
                continue;
            }
            if let Err(e) = peer.value().send(message.clone()).await {
                warn!("Failed to send message to {}: {}", peer.key(), e);
                // Remove the peer from the state if it's no longer reachable
                self.peers.remove(peer.key());
            }
        }
    }

    async fn add(
        &self,
        addr: SocketAddr,
        username: String,
        stream: Framed<TcpStream, LinesCodec>,
    ) -> Peer {
        let (tx, mut rx) = mpsc::channel(MAX_MESSAGES);
        self.peers.insert(addr, tx);

        // split the stream into a sender and a receiver
        let (mut stream_sender, stream_receiver) = stream.split();

        // receive messages from others, and send them to the client
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = stream_sender.send(message.to_string()).await {
                    warn!("Failed to send message to {}: {}", addr, e);
                    break;
                }
            }
        });

        // return peer
        Peer {
            username,
            stream: stream_receiver,
        }
    }
}

impl Message {
    fn user_joined(username: &str) -> Self {
        let content = format!("{username} has joined the chat");
        Self::UserJoined(content)
    }

    fn user_left(username: &str) -> Self {
        let content = format!("{username} has left the chat");
        Self::UserLeft(content)
    }

    fn chat(sender: impl Into<String>, content: impl Into<String>) -> Self {
        Self::Chat {
            sender: sender.into(),
            content: content.into(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::UserJoined(content) => write!(f, "\x1b[32m🟢 [系统] {content}\x1b[0m"),
            Message::UserLeft(content) => write!(f, "\x1b[31m🔴 [系统] {content}\x1b[0m"),
            Message::Chat { sender, content } => {
                write!(f, "\x1b[34m[{sender}]\x1b[0m {content}")
            }
        }
    }
}
