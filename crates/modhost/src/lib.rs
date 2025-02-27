#![doc = include_str!("../../../README.md")]
#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

pub use modhost_core::{Result, logger::*};
pub use modhost_server_core::{loader, loaders, models::*, tag, tags};

use axum::{Router, body::Bytes, extract::connect_info::IntoMakeServiceWithConnectInfo, serve};
use jsglue::{glue::Glue, util::is_debug};
use modhost_config::{AppConfig, get_config};
use modhost_db::{DbPool, create_connection, run_migrations};
use modhost_router::{create_api_spec, create_router};
use modhost_search::MeiliProject;
use modhost_server_core::{glue::make_glue, state::AppState, worker::run_worker};
use std::net::{IpAddr, SocketAddr};
use tokio::{join, net::TcpListener, task::JoinHandle};
use utoipa::openapi::OpenApi;

/// The main ModHost app.
/// This is the front-facing struct to allow users to
/// build their own apps around ModHost.
pub struct ModHost {
    /// The app configuration.
    config: AppConfig,

    /// The database pool.
    pool: DbPool,

    /// The [`Glue`] instance attached to the server.
    glue: Glue,

    /// The state object.
    state: AppState,

    /// The [`SocketAddr`] for the server to bind to.
    addr: SocketAddr,

    /// The server's OpenAPI spec.
    api_spec: OpenApi,

    /// The internal [`axum`] router.
    /// Will be [`Option::None`] until [`Self::router`] is called.
    router: Option<IntoMakeServiceWithConnectInfo<Router, SocketAddr>>,

    /// The join handle for the stats thread.
    #[allow(dead_code)]
    stats_thread: JoinHandle<Result<()>>,
}

impl ModHost {
    /// Create a new server instance.
    pub async fn new(verifier: Box<dyn Fn(Bytes) -> bool + Send + Sync>) -> Result<Self> {
        modhost_core::core_init();

        info!("Starting app...");
        info!("Getting config...");

        let config = get_config()?;

        info!("Connecting to the database (async pool)...");

        let pool = create_connection(Some(config.postgres.uri())).await?;

        info!("Creating state...");

        let api_spec = create_api_spec(&config);
        let state = AppState::new(pool.clone(), &config, verifier, api_spec.clone()).await?;

        info!("Running migrations...");

        run_migrations(&pool).await?;
        state.search.ensure_setup().await?;

        let index = state.search.projects();

        if index.get_documents::<MeiliProject>().await?.total == 0 {
            state.search.index_projects(&mut pool.get().await?).await?;
        }

        info!("Creating glue...");

        let glue = make_glue(&config).await?;

        info!("Starting stats thread...");

        let stats_thread = modhost_router::util::stats::start_stats_thread(&state);

        info!("Getting listen address...");

        let ip: IpAddr = config.server.host.parse()?;
        let addr = SocketAddr::from((ip, config.server.port));

        Ok(Self {
            config,
            pool,
            state,
            glue,
            addr,
            api_spec,
            router: None,
            stats_thread,
        })
    }

    /// Set the game versions for the API.
    pub fn versions(mut self, vers: Vec<GameVersion>) -> Self {
        self.state.game_versions = vers;
        self
    }

    /// Set the mod loaders for the API.
    pub fn loaders(mut self, loaders: Vec<ModLoader>) -> Self {
        self.state.loaders = loaders;
        self
    }

    /// Set the tags for the API.
    pub fn tags(mut self, tags: Vec<Tag>) -> Self {
        self.state.tags = tags;
        self
    }

    /// Register the router.
    /// - If you are registering versions, run this AFTER you run [`Self::versions`].
    /// - If you are registering loaders, run this AFTER you run [`Self::loaders`].
    /// - If you are registering tags, run this AFTER you run [`Self::tags`].
    pub fn router(mut self) -> Self {
        info!("Registering routes...");

        self.router = Some(
            create_router(&self.api_spec, self.state.clone(), self.glue.clone())
                .into_make_service_with_connect_info::<SocketAddr>(),
        );

        self
    }

    /// Run the server!
    pub async fn run(self) -> Result<()> {
        info!("Starting worker...");

        run_worker(self.pool);

        info!("Binding listener...");

        let listener = TcpListener::bind(&self.addr).await?;

        info!(
            "Started! Listening on {}:{}",
            self.config.server.host, self.config.server.port
        );

        let server = tokio::spawn(async move {
            serve(
                listener,
                self.router.expect(
                    "Router was not registered! Did you forget to run `ModHost::router()`?",
                ),
            )
            .await
        });

        if is_debug() {
            info!("Starting client...");

            let client = self.glue.spawn().await;
            let (a, b) = join!(client, server);

            a?;
            b??;

            return Ok(());
        }

        server.await??;

        Ok(())
    }
}
