//! Display packages which are installed.

const STATUS_FILE: &str = "/var/lib/dpkg/status";

use async_std::fs::File;
use asynchronous_codec::FramedRead;
use deb_control_codec::prelude::*;
use futures::{executor, prelude::*};
use std::str;
fn main() {
    executor::block_on(async_main());
}

async fn async_main() {
    let file = File::open(STATUS_FILE).await.unwrap();

    let mut control_stream = FramedRead::new(file, ControlDecoder::default());

    while let Some(event) = control_stream.next().await {
        let event = event.unwrap();
        let event = str::from_utf8(&event).expect("not UTF8");

        let mut control = Control::new(&event);

        let package = control.next().unwrap();

        println!("{}", package.value);
    }
}
