use std::env;
use std::sync::Arc;
use tokio::runtime::Builder as TokioBuilder;
use unftp_auth_rest::{Builder, RestAuthenticator};
use unftp_sbe_fs::ServerExt;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let _args: Vec<String> = env::args().collect();

    let authenticator: RestAuthenticator = Builder::new()
        .with_username_placeholder("{USER}".to_string())
        .with_password_placeholder("{PASS}".to_string())
        .with_url("http://127.0.0.1:8991/ftpd/auth".to_string())
        .with_method(hyper::Method::POST)
        .with_body(r#"{"username":"{USER}","password":"{PASS}"}"#.to_string())
        .with_selector("/status".to_string())
        .with_regex("pass".to_string())
        .build()?;

    let addr = "0.0.0.0:5151";
    let server = libunftp::Server::with_fs(std::env::temp_dir()).authenticator(Arc::new(authenticator));

    println!("Starting ftp server on {}", addr);
    let runtime = TokioBuilder::new_current_thread().enable_all().build()?;
    runtime.block_on(server.listen(addr))?;
    Ok(())
}
