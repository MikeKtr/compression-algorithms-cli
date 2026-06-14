use std::path::Path;

pub fn get_file_size(file_path: &Path) -> Result<u64, std::io::Error> {
    let md = std::fs::metadata(file_path)?;
    Ok(md.len())
}

pub fn print_stats(input: &Path, output: &Path, compression_algo: Option<&str>) {
    let len_input = get_file_size(input).unwrap_or(0);
    let len_output = get_file_size(output).unwrap_or(0);

    println!();
    println!("------------------------------------------------------");
    if let Some(algo) = compression_algo {
        println!(
            "Udało się zakończyć operację przy użyciu algorytmu {}",
            algo
        );
        println!("------------------------------------------------------");
    } else {
        println!("--- SUKCES ---");
    }

    println!("Początkowy rozmiar pliku: {} bajtów", len_input);
    println!("Końcowy rozmiar pliku:    {} bajtów", len_output);

    if len_input > 0 {
        let percent = (len_output as f64 / len_input as f64) * 100.0;
        let reduced = 100.0 - percent;
        println!("Rozmiar po operacji:     {:.2}% oryginału", percent);
        println!("Zredukowano o:            {:.2}%", reduced);
    }
    println!("------------------------------------------------------");
}
