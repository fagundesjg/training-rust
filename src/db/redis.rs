use redis::{Client, aio::MultiplexedConnection};
use std::env;

pub async fn init_redis() -> MultiplexedConnection {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL não definido");
    let client = Client::open(redis_url).expect("Erro ao criar cliente Redis");

    client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Erro ao conectar no Redis")
}
