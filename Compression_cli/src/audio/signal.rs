// const COMPRESSION_FACTOR: i16 = 1000;
const COMPRESSION_FACTOR: i16 = 2000;

// potrzebne, zeby RLE mialo sens, cos w stylu zaokrąglenia wartosci dzwiekow
pub fn quantize_samples(samples: &mut [i16]) {
    for sample in samples.iter_mut() {
        *sample = (*sample / COMPRESSION_FACTOR) * COMPRESSION_FACTOR;
    }
}

// [10, 12, 14, 16] -> [10, 2, 2, 2]; przez to mozna robic RLE
pub fn delta_encode(samples: &[i16]) -> Vec<i16> {
    let mut encoded = Vec::with_capacity(samples.len());

    if let Some(&first) = samples.first() {
        encoded.push(first);
    }

    for i in 1..samples.len() {
        let diff = samples[i].wrapping_sub(samples[i - 1]);
        encoded.push(diff);
    }

    encoded
}

pub fn delta_decode(encoded: &[i16]) -> Vec<i16> {
    let mut decoded = Vec::with_capacity(encoded.len());

    if let Some(&first) = encoded.first() {
        decoded.push(first);

        for i in 1..encoded.len() {
            let next_val = decoded[i - 1].wrapping_add(encoded[i]);
            decoded.push(next_val);
        }
    }

    decoded
}
