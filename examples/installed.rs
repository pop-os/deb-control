//! Display packages which are installed.

#[macro_use]
extern crate fomat_macros;

const STATUS_FILE: &str = "/var/lib/dpkg/status";

use async_std::fs::File;
use deb_control::prelude::*;
use futures::{executor, prelude::*};
use futures_codec::FramedRead;
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

        pintln!((package.value));
    }
}
