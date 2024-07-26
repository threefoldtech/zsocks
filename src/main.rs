extern crate log;

use clap::Parser;
use fast_socks5::{
    server::{Config, SimpleUserPassword, Socks5Server, Socks5Socket},
    Result,
};
use log::{error, info};
use std::future::Future;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::task;
use tokio_stream::StreamExt;

// socks5 proxy server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Opt {
    /// Bind on address address. eg. `127.0.0.1:9800`
    #[arg(short, long, env = "LISTEN_ADDR", default_value = "127.0.0.1:9800")]
    pub listen_addr: String,

    /// Request timeout
    #[arg(short, long, env = "REQUEST_TIMEOUT", default_value = "10")]
    pub request_timeout: u64,

    /// Username
    #[arg(long, env = "USERNAME", required = true)]
    pub username: String,

    /// Password
    #[arg(long, env = "PASSWORD", required = true)]
    pub password: String,

    /// Don't perform the auth handshake, send directly the command request
    #[arg(short, long, default_value_t = false)]
    pub skip_auth: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt: Opt = Opt::parse();
    env_logger::init();

    spawn_socks_server(opt).await
}

async fn spawn_socks_server(opt: Opt) -> Result<()> {
    let mut config: Config<SimpleUserPassword> = Config::default();
    config.set_request_timeout(opt.request_timeout);
    config.set_skip_auth(opt.skip_auth);
    config.set_dns_resolve(true);
    let config = config.with_authentication(SimpleUserPassword {
        username: opt.username,
        password: opt.password,
    });

    let listener = <Socks5Server>::bind(&opt.listen_addr).await?;
    let listener = listener.with_config(config);

    let mut incoming = listener.incoming();

    info!("Listen for socks connections @ {}", &opt.listen_addr);

    // Standard TCP loop
    while let Some(socket_res) = incoming.next().await {
        match socket_res {
            Ok(socket) => {
                spawn_and_log_error(socket.upgrade_to_socks5());
            }
            Err(err) => {
                error!("accept error = {:?}", err);
            }
        }
    }

    Ok(())
}

fn spawn_and_log_error<F, T>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<Socks5Socket<T, SimpleUserPassword>>> + Send + 'static,
    T: AsyncRead + AsyncWrite + Unpin,
{
    task::spawn(async move {
        match fut.await {
            Ok(mut socket) => {
                if let Some(user) = socket.take_credentials() {
                    info!("user logged in with `{}`", user.username);
                }
            }
            Err(err) => error!("{:#}", &err),
        }
    })
}
