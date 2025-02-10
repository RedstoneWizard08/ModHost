//! A macro library to enable quick and easy creation of ModHost-based apps.

pub extern crate axum;
pub extern crate clap;
pub extern crate clap_complete;
pub extern crate clap_verbosity_flag;
pub extern crate dotenvy;
pub extern crate modhost;
pub extern crate tokio;

/// The QuickHost macro.
///
/// Example:
///
/// ```rs
/// use axum::body::Bytes;
/// use modhost::{GameVersion, Result};
///
/// async fn some_function_that_returns_versions() -> Result<Vec<GameVersion>> {
///     Ok(vec![])
/// }
///
/// fn verify_project_file(_bytes: Bytes) -> bool {
///     // Do something with the bytes or just return true
///     // This will tell ModHost whether the project file is valid or not.
///     true
/// }
///
/// quickhost::quickhost! {
///     versions = [crate::some_function_that_returns_versions().await?];
///     loaders = [modhost::loaders!["This", "Can", "Also", "Be", "A", "Function", "Like", "Above"]];
///     tags = [modhost::tags![]]; // This can also be omitted if you don't have tags.
///     verifier = [crate::verify_project_file]; // This line can be omitted to always return true.
/// }
/// ```
#[macro_export]
macro_rules! quickhost {
    {
        versions = [$($versions: tt)*];
        loaders = [$($loaders: tt)*];
        $(tags = [$($tags: tt)*];)?
        verifier = [$($verifier: tt)*];
    } => {
        mod __quickhost_impl {
            use $crate::clap::{self, Parser, CommandFactory, Command};
            use $crate::clap_verbosity_flag::{Verbosity, InfoLevel};
            use $crate::clap_complete::{Shell, Generator, generate};
            use $crate::modhost::{ModHost, Result, init_logger, from_log_level};
            use $crate::dotenvy::dotenv;
            use std::io::stdout;

            #[derive(Debug, Clone, Parser)]
            #[command(version, about, long_about = None)]
            struct QuickHostCli {
                #[command(flatten)]
                verbose: Verbosity<InfoLevel>,

                #[arg(short = 'C', long)]
                complete: Option<Shell>,
            }

            impl QuickHostCli {
                fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
                    generate(generator, cmd, cmd.get_name().to_string(), &mut stdout());
                }

                pub async fn run(self) -> Result<()> {
                    if let Some(shell) = self.complete {
                        Self::print_completions(shell, &mut Self::command());
                        return Ok(());
                    }

                    let _ = dotenv();

                    let _guard = init_logger("modhost-server", from_log_level(self.verbose.log_level_filter()))?;

                    ModHost::new(Box::new($($verifier)*))
                        .await?
                        .versions($($versions)*)
                        .loaders($($loaders)*)
                        $(.tags($($tags)*))?
                        .router()
                        .run()
                        .await?;

                    Ok(())
                }
            }

            pub async fn run() -> Result<()> {
                QuickHostCli::parse().run().await
            }
        }

        pub fn main() -> $crate::modhost::Result<()> {
            $crate::tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(__quickhost_impl::run())
        }
    };

    {
        versions = [$($versions: tt)*];
        loaders = [$($loaders: tt)*];
        $(tags = [$($tags: tt)*];)?
    } => {
        fn __gen_verify(_: $crate::axum::body::Bytes) -> bool { true }

        $crate::quickhost! {
            versions = [$($versions)*];
            loaders = [$($loaders)*];
            $(tags = [$($tags)*];)?
            verifier = [super::__gen_verify];
        }
    };
}
