#![cfg(feature = "kafka")]

extern crate rusoto_core;
extern crate rusoto_kafka;

use rusoto_core::Region;
use rusoto_kafka::{Kafka, KafkaClient, ListClustersRequest};

#[test]
fn should_list_clusters() {
    let client = KafkaClient::new(Region::UsEast1);
    let request = ListClustersRequest::default();

    let response = client.list_clusters(request).sync().unwrap();
    println!("Got response: {:?}", response);
}
