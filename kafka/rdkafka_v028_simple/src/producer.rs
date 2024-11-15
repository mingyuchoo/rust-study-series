use rdkafka::producer::{BaseProducer,
                        BaseRecord};
use rdkafka::ClientConfig;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let producer: BaseProducer = ClientConfig::new().set("bootstrap.servers", "localhost:9092")
                                                    .create()
                                                    .expect("invalid producer config");

    for i in 1 .. 100 {
        println!("sending message");

        producer.send(BaseRecord::to("rust").key(&format!("key-{}", i))
                                            .payload(&format!("value-{}", i)))
                .expect("failed to send message");

        thread::sleep(Duration::from_secs(3));
    }

    Ok(())
}
