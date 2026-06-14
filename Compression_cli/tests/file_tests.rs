#[cfg(test)]
mod tests {
    use compression_cli::algorithms::huffman_tree::HuffmanCompression;
    use compression_cli::algorithms::lzw::LzwCompression;
    use compression_cli::algorithms::rle::RleCompression;
    use compression_cli::algorithms::traits::CompressionAlgorithm;
    use compression_cli::image::png::PngCompression;

    use std::fs;
    use std::io::Cursor;

    fn test_roundtrip(algo: &dyn CompressionAlgorithm, file_path: &str) {
        let original_data = fs::read(file_path).expect("missing file");

        let mut source = Cursor::new(original_data.clone());
        let mut compressed_data = Cursor::new(Vec::new());

        algo.compress(&mut source, &mut compressed_data)
            .expect("compression error");

        compressed_data.set_position(0);
        let mut decompressed_data = Cursor::new(Vec::new());

        algo.decompress(&mut compressed_data, &mut decompressed_data)
            .expect("decompression error");

        assert_eq!(
            original_data,
            decompressed_data.into_inner(),
            "decompressed data differs from original"
        );
    }

    #[test]
    fn test_lzw_shakespire() {
        test_roundtrip(&LzwCompression, "shakespire.txt");
    }

    #[test]
    fn test_huffman_shakespire() {
        test_roundtrip(&HuffmanCompression, "shakespire.txt");
    }

    #[test]
    fn test_rle_shakespire() {
        test_roundtrip(&RleCompression, "shakespire.txt");
    }

    #[test]
    fn test_png_bmp() {
        test_roundtrip(&PngCompression, "greenland_grid_velo.bmp");
    }
}
