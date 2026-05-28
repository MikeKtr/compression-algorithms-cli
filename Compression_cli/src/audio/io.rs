use hound::{WavReader, WavSpec, WavWriter};
use std::path::Path;

pub fn load_samples(path: &Path) -> Result<Vec<i16>, String> {
    let mut reader = WavReader::open(path).map_err(|e| format!("Błąd otwierania WAV: {}", e))?;

    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap_or(0)).collect();

    if samples.is_empty() {
        return Err("Plik WAV jest pusty".to_string());
    }

    let max_val = samples.iter().map(|s| s.abs()).max().unwrap_or(0);
    if max_val == 0 {
        return Err("Plik WAV jest bledny".to_string());
    }

    Ok(samples)
}

pub fn save_samples(path: &Path, samples: &[i16]) -> Result<(), String> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer =
        WavWriter::create(path, spec).map_err(|e| format!("Błąd tworzenia pliku WAV: {}", e))?;

    for &sample in samples {
        writer
            .write_sample(sample)
            .map_err(|e| format!("blad zapisu pliku: {}", e))?;
    }

    writer
        .finalize()
        .map_err(|e| format!("blad zapisu pliku: {}", e))?;

    Ok(())
}
