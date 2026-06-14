# Compression CLI

Compression CLI to narzędzie wiersza poleceń (CLI) przeznaczone do kompresji i dekompresji plików przy użyciu różnych algorytmów. Aplikacja obsługuje wiele technik kompresji danych, takich jak kodowanie Huffmana, RLE (Run-Length Encoding) oraz LZW, a także udostępnia narzędzia do przetwarzania plików audio oraz obrazów.

## Funkcje

- **Algorytmy kompresji**:
  - Kodowanie Huffmana
  - Kodowanie RLE (Run-Length Encoding)
  - Kompresja LZW
- **Przetwarzanie audio**:
  - Przetwarzanie sygnałów i narzędzia wejścia/wyjścia (I/O)
  - Generowanie wykresów audio
- **Przetwarzanie obrazu**:
  - Obsługa plików formatu PNG

Projekt charakteryzuje się modułową i rozszerzalną architekturą, co pozwala na łatwe dodawanie nowych algorytmów kompresji.

## Wymagania i instalacja

Do uruchomienia projektu wymagane jest środowisko programistyczne języka Rust (**Cargo**).

Projekt ten obsługuje 3 główne komendy (`file, image, audio`) słuzace do demonstracji działania algorytmów na róznych typach plików.
Dzięki uzyciu biblioteki `clap` program obsługuje takze flagę --help/-h która moze pomóc w obsłudze programu.
Przykład

```bash
cargo run file -h

Usage: compression_cli file [OPTIONS] <INPUT> <OUTPUT> <COMPRESSION_ALGO>

Arguments:
  <INPUT>
  <OUTPUT>
  <COMPRESSION_ALGO>  [possible values: rle, huffman-compression, lzw, png]

Options:
  -d, --decompress
  -h, --help        Print help
```

Przykład uzycia programu

```bash
# Kompresja pliku tekstowego przy użyciu drzewa Huffmana
cargo run file shakespire.txt shakespire.huffman huffman-compression

------------------------------------------------------
Udało się zakończyć operację przy użyciu algorytmu HuffmanCompression
------------------------------------------------------
Początkowy rozmiar pliku: 5562284 bajtów
Końcowy rozmiar pliku:    3385129 bajtów
Rozmiar po operacji:     60.86% oryginału
Zredukowano o:            39.14%
------------------------------------------------------
```

Dekompresja

```bash
cargo run file shakespire.huffman shakespire_decompressed.txt huffman-compression -d

------------------------------------------------------
Udało się zakończyć operację przy użyciu algorytmu HuffmanCompression
------------------------------------------------------
Początkowy rozmiar pliku: 3385129 bajtów
Końcowy rozmiar pliku:    5562284 bajtów
Rozmiar po operacji:     164.32% oryginału
Zredukowano o:            -64.32%
------------------------------------------------------
```

W rezultacie powstał tez plik `shakespire_decompressed.txt`, który jest identyczny z oryginałem

```bash
# Konwersja/kompresja pliku BMP do formatu PNG
cargo run image greenland_grid_velo.bmp c_green.png

------------------------------------------------------
Udało się zakończyć operację przy użyciu algorytmu Png Compression
------------------------------------------------------
Początkowy rozmiar pliku: 2995046 bajtów
Końcowy rozmiar pliku:    1002433 bajtów
Rozmiar po operacji:     33.47% oryginału
Zredukowano o:            66.53%
------------------------------------------------------
```

```bash

# Kompresja pliku audio WAV przy użyciu algorytmu LZW
# Przez uzycie kwantyzacji, kompresja audio jest stratna
cargo run audio audio.wav c_sample.lzw lzw

```

Poza komendami które wykonują algorytmy, została równiez dodana komenda `explain`, która pozwala zrozumieć ogólną ideę danego algorytmu

```bash
cargo run explain rle

=== Rle Compression ===
Compresses data by replacing consecutive identical bytes with a [count, value] pair.
Example: 'AAAAABBB' is compressed into [5, 'A', 3, 'B']
```

## Uruchamianie testów

Projekt zawiera zestaw testów sprawdzających poprawność kompresji i dekompresji róznych algorytmów

Uruchomienie testów:

```bash
cargo test
```

## Refleksje nad językiem Rust

### Co ułatwiło implementację?

    - Precyzyjna kontrola nad typami numerycznymi, podział na typy u8, u16, u64 itp.

    - Inferencja typów: Zaawansowane wnioskowanie typów przez kompilator.

    - Cechy: Dzięki uzyciu jednej wspólnej cechy `CompressionAlgorithm` główny program nie musi znać szczegółów każdego algorytmu z osobna – wystarczy, że wie, iż na każdym z nich można wywołać operację `compress` i `decompress`. Bardzo ułatwiło to dodawanie kolejnych algorytmów, a takze pozwala na potencjalne rozszerzenie programu w przyszłości

    - Gotowe biblioteki: Dzięki bibliotece `clap`, tworzenie menu komend było proste. Rust wygenerował za nas bezpieczny interfejs obsługujący wszystkie argumenty i automatycznie dodał panel pomocy (`--help`). Innymi bibliotekami wykorzystywanymi w projekcie są np: `hound`, który ułatwia odczyt plików wav, czy `textplots` który pozwolił na narysowanie sygnału audio w ASCII

    - Operator "?": prz. ` reader.read_exact(&mut file_hdr)?;`. Słuzy on do szybkiego propagowania błędów wyzej, zamiast pisania dodatkowych instrukcji warunkowych

### Co stanowiło największe wyzwanie?

    - Rygorystyczny system prawa własności (Ownership) przy operacjach I/O.
