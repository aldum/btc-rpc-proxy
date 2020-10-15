use anyhow::Error;
use btc_rpc_proxy;

mod create_env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = create_env::create_env().await?.leak();
    btc_rpc_proxy::main(env).await
}
