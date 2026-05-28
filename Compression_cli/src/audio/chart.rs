use textplots::{Chart, Plot, Shape};

// wykres ascii audio
pub fn show_waveform(samples: &[i16]) {
    const NUM_POINTS: usize = 300;

    let step = std::cmp::max(1, samples.len() / NUM_POINTS);

    let max_sample = samples
        .iter()
        .map(|&s| s.abs() as f32)
        .fold(0.0f32, f32::max);

    let divisor = if max_sample > 0.0 { max_sample } else { 1.0 };

    let wave_data: Vec<(f32, f32)> = (0..NUM_POINTS)
        .filter_map(|i| {
            let index = i * step;
            samples.get(index).map(|&s| (i as f32, s as f32 / divisor))
        })
        .collect();

    Chart::new(420, 170, 0.0, NUM_POINTS as f32)
        .lineplot(&Shape::Lines(&wave_data))
        .display();
}
