//! The upgrade request.

use crate::{
    fail::{DefaultOnFailedUpgrade, OnFailedUpgrade},
    rejection::{
        ConnectionNotUpgradable, InvalidConnectionHeader, InvalidUpgradeHeader,
        InvalidWebSocketVersionHeader, MethodNotGet, WebSocketKeyHeaderMissing,
        WebSocketUpgradeRejection,
    },
    socket::WebSocket,
    util::{header_contains, header_eq, sign},
};
use async_trait::async_trait;
use axum_core::{body::Body, extract::FromRequestParts, response::Response, Error};
use futures_util::Future;
use http::{header, request::Parts, HeaderValue, Method, StatusCode};
use hyper_util::rt::TokioIo;
use std::borrow::Cow;
use tokio_tungstenite::{
    tungstenite::protocol::{self, WebSocketConfig},
    WebSocketStream,
};

/// A WebSocket upgrade request.
pub struct WebSocketUpgrade<F = DefaultOnFailedUpgrade> {
    config: WebSocketConfig,
    protocol: Option<HeaderValue>,
    sec_websocket_key: HeaderValue,
    on_upgrade: hyper::upgrade::OnUpgrade,
    on_failed_upgrade: F,
    sec_websocket_protocol: Option<HeaderValue>,
}

impl<F> std::fmt::Debug for WebSocketUpgrade<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebSocketUpgrade")
            .field("config", &self.config)
            .field("protocol", &self.protocol)
            .field("sec_websocket_key", &self.sec_websocket_key)
            .field("sec_websocket_protocol", &self.sec_websocket_protocol)
            .finish_non_exhaustive()
    }
}

impl<F> WebSocketUpgrade<F> {
    /// Set the write buffer's size.
    pub fn write_buffer_size(mut self, size: usize) -> Self {
        self.config.write_buffer_size = size;
        self
    }

    /// Set the maximum write buffer's size.
    pub fn max_write_buffer_size(mut self, max: usize) -> Self {
        self.config.max_write_buffer_size = max;
        self
    }

    /// Set the maximum message size.
    pub fn max_message_size(mut self, max: usize) -> Self {
        self.config.max_message_size = Some(max);
        self
    }

    /// Set the maximum frame size.
    pub fn max_frame_size(mut self, max: usize) -> Self {
        self.config.max_frame_size = Some(max);
        self
    }

    /// Toggle accepting unmasked frames.
    pub fn accept_unmasked_frames(mut self, accept: bool) -> Self {
        self.config.accept_unmasked_frames = accept;
        self
    }

    /// Set the protocols to allow.
    pub fn protocols<I>(mut self, protocols: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Cow<'static, str>>,
    {
        if let Some(req_protocols) = self
            .sec_websocket_protocol
            .as_ref()
            .and_then(|p| p.to_str().ok())
        {
            self.protocol = protocols
                .into_iter()
                // FIXME: This will often allocate a new `String` and so is less efficient than it
                // could be. But that can't be fixed without breaking changes to the public API.
                .map(Into::into)
                .find(|protocol| {
                    req_protocols
                        .split(',')
                        .any(|req_protocol| req_protocol.trim() == protocol)
                })
                .map(|protocol| match protocol {
                    Cow::Owned(s) => HeaderValue::from_str(&s).unwrap(),
                    Cow::Borrowed(s) => HeaderValue::from_static(s),
                });
        }

        self
    }

    /// Set the callback for when an upgrade fails.
    pub fn on_failed_upgrade<C>(self, callback: C) -> WebSocketUpgrade<C>
    where
        C: OnFailedUpgrade,
    {
        WebSocketUpgrade {
            config: self.config,
            protocol: self.protocol,
            sec_websocket_key: self.sec_websocket_key,
            on_upgrade: self.on_upgrade,
            on_failed_upgrade: callback,
            sec_websocket_protocol: self.sec_websocket_protocol,
        }
    }

    /// Set the callback for when a WebSocket upgrades.
    #[must_use = "to set up the WebSocket connection, this response must be returned"]
    pub fn on_upgrade<C, Fut>(self, callback: C) -> Response
    where
        C: FnOnce(WebSocket) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
        F: OnFailedUpgrade,
    {
        let on_upgrade = self.on_upgrade;
        let config = self.config;
        let on_failed_upgrade = self.on_failed_upgrade;

        let protocol = self.protocol.clone();

        tokio::spawn(async move {
            let upgraded = match on_upgrade.await {
                Ok(upgraded) => upgraded,
                Err(err) => {
                    on_failed_upgrade.call(Error::new(err));
                    return;
                }
            };
            let upgraded = TokioIo::new(upgraded);

            let socket =
                WebSocketStream::from_raw_socket(upgraded, protocol::Role::Server, Some(config))
                    .await;
            let socket = WebSocket {
                inner: socket,
                protocol,
            };
            callback(socket).await;
        });

        #[allow(clippy::declare_interior_mutable_const)]
        const UPGRADE: HeaderValue = HeaderValue::from_static("upgrade");
        #[allow(clippy::declare_interior_mutable_const)]
        const WEBSOCKET: HeaderValue = HeaderValue::from_static("websocket");

        let mut builder = Response::builder()
            .status(StatusCode::SWITCHING_PROTOCOLS)
            .header(header::CONNECTION, UPGRADE)
            .header(header::UPGRADE, WEBSOCKET)
            .header(
                header::SEC_WEBSOCKET_ACCEPT,
                sign(self.sec_websocket_key.as_bytes()),
            );

        if let Some(protocol) = self.protocol {
            builder = builder.header(header::SEC_WEBSOCKET_PROTOCOL, protocol);
        }

        builder.body(Body::empty()).unwrap()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for WebSocketUpgrade<DefaultOnFailedUpgrade>
where
    S: Send + Sync,
{
    type Rejection = WebSocketUpgradeRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if parts.method != Method::GET {
            return Err(MethodNotGet.into());
        }

        if !header_contains(&parts.headers, header::CONNECTION, "upgrade") {
            return Err(InvalidConnectionHeader.into());
        }

        if !header_eq(&parts.headers, header::UPGRADE, "websocket") {
            return Err(InvalidUpgradeHeader.into());
        }

        if !header_eq(&parts.headers, header::SEC_WEBSOCKET_VERSION, "13") {
            return Err(InvalidWebSocketVersionHeader.into());
        }

        let sec_websocket_key = parts
            .headers
            .get(header::SEC_WEBSOCKET_KEY)
            .ok_or(WebSocketKeyHeaderMissing)?
            .clone();

        let on_upgrade = parts
            .extensions
            .remove::<hyper::upgrade::OnUpgrade>()
            .ok_or(ConnectionNotUpgradable)?;

        let sec_websocket_protocol = parts.headers.get(header::SEC_WEBSOCKET_PROTOCOL).cloned();

        Ok(Self {
            config: Default::default(),
            protocol: None,
            sec_websocket_key,
            on_upgrade,
            sec_websocket_protocol,
            on_failed_upgrade: DefaultOnFailedUpgrade,
        })
    }
}
