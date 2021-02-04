use elasticsearch::{ Elasticsearch};
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::cat::CatIndicesParts;

#[tokio::main]
async fn main() -> () {
    let url = Url::parse("XXX").unwrap();
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).build().unwrap();
    let client = Elasticsearch::new(transport);

    let response = client
        .cat()
        .indices(CatIndicesParts::None)
        .v(true)
        .send()
        .await.unwrap();

    let text = response.text().await.unwrap();
    println!("{}", text);
}
