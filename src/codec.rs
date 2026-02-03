use asynchronous_codec::Decoder;
use bytes::BytesMut;
use std::io;

#[derive(Default)]
pub struct ControlDecoder;

impl Decoder for ControlDecoder {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match memchr::memmem::find_iter(src.as_ref(), "\n\n").next() {
            Some(pos) => {
                let buf = src.split_to(pos + 2);
                Ok(Some(buf))
            }
            None => Ok(None),
        }
    }

    fn decode_eof(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let buf = src.split_to(src.len());
        Ok(Some(buf))
    }
}
