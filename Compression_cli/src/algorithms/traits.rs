use std::io::{Read, Write,Seek};


pub trait ReadSeek: Read + Seek {}

impl<T: Read + Seek + ?Sized> ReadSeek for T {}

pub trait CompressionAlgorithm {

    fn compress(&self,source: &mut dyn ReadSeek, destination: &mut dyn Write) ->  std::io::Result<()>;
    fn decompress(&self, source: &mut dyn ReadSeek, destination: &mut dyn Write) -> std::io::Result<()>;
   
    fn name(&self) -> &'static str;
}