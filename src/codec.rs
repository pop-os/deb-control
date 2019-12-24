use bytes::BytesMut;
use futures_codec::Decoder;
use std::io;

#[derive(Default)]
pub struct ControlDecoder;

impl Decoder for ControlDecoder {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match twoway::find_bytes(src, b"\n\n") {
            Some(pos) => {
                let buf = src.split_to(pos + 2);
                Ok(Some(buf))
            }
            None => Ok(None),
        }
    }
}
