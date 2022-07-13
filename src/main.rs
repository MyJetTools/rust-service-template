use rust_service_template::telemetry::{get_subscriber, init_subscriber, ElasticSink};
use std::{
    io::{self, Write},
    time::Duration,
};
use tracing::info;
use tracing::instrument;

#[tokio::main]
async fn main() {
    //let mut sink = SINK;
    let sink = ElasticSink::new("127.0.0.1:7878".to_string().parse().unwrap());
    let mut writer = sink.create_writer();
    let subscriber = get_subscriber("rust_service_template".into(), "info".into(), move || {
        sink.create_writer()
    });
    init_subscriber(subscriber);

    let mut counter = 0;
    while (counter < 20) {
        some_logic(Example { a: counter, b: 2 });
        tokio::time::sleep(Duration::from_millis(1000)).await;
        counter += 1;
    }
}

struct Example {
    a: i32,
    b: i32,
}

#[instrument(skip(example), fields(example.a = %example.a))]
fn some_logic(example: Example) {
    info!("EXECUTING");
}
