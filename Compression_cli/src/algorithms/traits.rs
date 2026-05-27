use std::io::{Read, Write};


pub trait CompressionAlgorithm {

    fn compress(&self,source: &mut dyn Read, destination: &mut dyn Write) ->  std::io::Result<()>;
    fn decompress(&self, source: &mut dyn Read, destination: &mut dyn Write) -> std::io::Result<()>;
   
    fn name(&self) -> &'static str;
}