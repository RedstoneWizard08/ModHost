use crate::{
    glue::make_glue,
    routes::{
        create_router,
        meta::{loaders::ModLoader, vers::GameVersion},
    },
    state::AppState,
    worker::run_worker,
};
use anyhow::Result;
use app_config::{get_config, AppConfig};
use axum::{body::Bytes, extract::connect_info::IntoMakeServiceWithConnectInfo, serve, Router};
use db::{create_connection, run_migrations, DbPool};
use jsglue::{glue::Glue, util::is_debug};
use std::net::{IpAddr, SocketAddr};
use tokio::{join, net::TcpListener};

pub struct ModHost {
    config: AppConfig,
    pool: DbPool,
    glue: Glue,
    state: AppState,
    addr: SocketAddr,
    router: Option<IntoMakeServiceWithConnectInfo<Router, SocketAddr>>,
}

impl ModHost {
    /// Create a new server instance.
    pub async fn new(verifier: Box<dyn Fn(Bytes) -> bool + Send + Sync>) -> Result<Self> {
        info!("Starting app...");
        info!("Getting config...");

        let config = get_config()?;

        info!("Connecting to the database (async pool)...");

        let pool = create_connection(Some(config.postgres.uri())).await?;

        info!("Running migrations...");

        run_migrations(&pool).await?;

        info!("Creating state...");

        let state = AppState::new(pool.clone(), &config, verifier)?;

        info!("Creating glue...");

        let glue = make_glue(&config).await?;

        info!("Getting listen address...");

        let ip: IpAddr = config.server.host.parse()?;
        let addr = SocketAddr::from((ip, config.server.port));

        Ok(Self {
            config,
            pool,
            state,
            glue,
            addr,
            router: None,
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

    /// Register the router.
    /// If you are registering versions, run this AFTER you run [`Self::versions`]!
    pub fn router(mut self) -> Self {
        info!("Registering routes...");

        self.router = Some(
            create_router(self.state.clone(), self.glue.clone())
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
