use clap::Subcommand;

mod connector;
mod gateway;
mod media;

pub use connector::run_media_connector;
pub use gateway::run_media_gateway;
pub use media::run_media_server;

#[derive(Debug, Subcommand)]
pub enum ServerType {
    Gateway(gateway::Args),
    Connector(connector::Args),
    Media(media::Args),
}
