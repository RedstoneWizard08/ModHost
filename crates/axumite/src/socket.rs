//! The WebSocket wrapper.

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum_core::Error;
use futures_util::{Sink, SinkExt, Stream, StreamExt};
use http::HeaderValue;
use hyper_util::rt::TokioIo;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

/// A WebSocket.
#[derive(Debug)]
pub struct WebSocket {
    /// The inner [`WebSocketStream`].
    pub(crate) inner: WebSocketStream<TokioIo<hyper::upgrade::Upgraded>>,

    /// The protocol used to connect.
    pub(crate) protocol: Option<HeaderValue>,
}

impl WebSocket {
    /// Receive a [`Message`] asynchronously.
    /// This will wait for the next message to be received before returning.
    pub async fn recv(&mut self) -> Option<Result<Message, Error>> {
        self.next().await
    }

    /// Send a [`Message`] asynchronously.
    pub async fn send(&mut self, msg: Message) -> Result<(), Error> {
        self.inner.send(msg).await.map_err(Error::new)
    }

    /// Close the WebSocket, sending the close message.
    pub async fn close(mut self) -> Result<(), Error> {
        self.inner.close(None).await.map_err(Error::new)
    }

    /// Get the protocol used to connect.
    pub fn protocol(&self) -> Option<&HeaderValue> {
        self.protocol.as_ref()
    }
}

impl Stream for WebSocket {
    type Item = Result<Message, Error>;

    #[allow(clippy::never_loop)]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match futures_util::ready!(self.inner.poll_next_unpin(cx)) {
                Some(Ok(msg)) => {
                    return Poll::Ready(Some(Ok(msg)));
                }
                Some(Err(err)) => return Poll::Ready(Some(Err(Error::new(err)))),
                None => return Poll::Ready(None),
            }
        }
    }
}

impl Sink<Message> for WebSocket {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_ready(cx).map_err(Error::new)
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        Pin::new(&mut self.inner)
            .start_send(item)
            .map_err(Error::new)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx).map_err(Error::new)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_close(cx).map_err(Error::new)
    }
}
