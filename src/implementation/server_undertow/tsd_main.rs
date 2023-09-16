use log;

use super::command::build_command;
use crate::core::coredb::default_tsdb::DefaultTSDB;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = build_command();
    let port = config.get_port();
    let ssl_port = config.get_ssl_port();
    if port < 1 || ssl_port < 1 {
        log::error!("Must provide a HTTP or SSL port.");
        return Err(anyhow::anyhow!("Crash because port or ssl port wrong."));
    }

    let bind = config.get_bind();
    let root = config.get_root();
    let load_plugins = config.get_load_plugins();

    let tsdb = DefaultTSDB::new(config);

    if load_plugins {
        let res = tsdb.initialize_registry(true).await;

        if res.is_err() {
            return Err(anyhow::anyhow!("Failed to initialize TSDB registry"));
        }
    }

    // Make sure shutdown gracefully.
    // register_shutdown_hook(server, tsdb);

    // build http server with tsdb

    // check if config has directory key

    // load auth

    // deploy tsdb

    // start http handler

    // start query handler

    // start encoding handler

    // start stats collector handler

    // start rewrite handler

    // start access log handler

    // real start http listener

    // setup ssl

    // start

    Err(anyhow::anyhow!("Nothing to do"))
}
