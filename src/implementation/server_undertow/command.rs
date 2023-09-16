use clap::{Command, Parser};

use crate::common::configuration::configuration::Configuration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(long, default_value_t = 6667)]
    pub http_port_key: i32,
    #[arg(long, default_value_t = 7776)]
    tls_port_key: i32,
    #[arg(long, default_value = "0.0.0.0", required = true)]
    bind_key: String,
    #[arg(long, default_value = "/", required = true)]
    root_key: String,
    #[arg(long, default_value = "true", required = true)]
    load_plugins_key: bool,
    #[arg(long)]
    keystore_key: String,
    #[arg(long)]
    tls_cert_key: String,
    #[arg(long)]
    tls_key_key: String,
    #[arg(long)]
    keystore_pass_key: String,
    #[arg(long)]
    tls_secret_cert_key: String,
    #[arg(long)]
    tls_secret_key_key: String,
    #[arg(long)]
    tls_ca_key: String,
    #[arg(long)]
    tls_verify_client_key: String,
    #[arg(long)]
    tls_protocols_key: String,
    #[arg(long)]
    tls_cipher_key: String,
    #[arg(long)]
    directory_key: String,
    #[arg(long, default_value_t = 5 * 60 * 1000)]
    read_to_key: i32,
    #[arg(long, default_value_t = 5 * 60 * 1000)]
    write_to_key: i32,
    #[arg(long, default_value_t = 15 * 60 * 1000)]
    no_request_key: i32,
    #[arg(long, default_value = "combined")]
    access_log_format_key: String,
    #[arg(long)]
    record_request_start_time: String,
    #[arg(long)]
    rewrite_key: String,
}

pub fn build_command() -> Configuration {
    let arg = Args::parse();
    let mut config = Configuration::new();

    config
}
