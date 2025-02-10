use futures_util::{Sink, SinkExt, stream::SplitSink};
use std::{fmt::Debug, net::SocketAddr, ops::ControlFlow};
use tokio_tungstenite::tungstenite::Message;

pub async fn process_message<T, E>(
    socket: &mut SplitSink<T, Message>,
    who: SocketAddr,
    message: Message,
) -> ControlFlow<(), ()>
where
    T: Sink<Message, Error = E>,
    E: Debug,
{
    match message {
        Message::Text(txt) => {
            socket.send(Message::Text(txt)).await.unwrap();
        }

        Message::Binary(bin) => {
            socket.send(Message::Binary(bin)).await.unwrap();
        }

        Message::Close(c) => {
            if let Some(cf) = c.clone() {
                info!(
                    "{} sent close with code {} and reason `{}`.",
                    who, cf.code, cf.reason
                );
            } else {
                info!("{who} somehow sent close message without CloseFrame!");
            }

            socket.send(Message::Close(c)).await.unwrap();
            socket.close().await.unwrap();

            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            socket.send(Message::Pong(v)).await.unwrap();
        }

        Message::Ping(v) => {
            socket.send(Message::Ping(v)).await.unwrap();
        }

        Message::Frame(_) => unreachable!(),
    }

    ControlFlow::Continue(())
}
