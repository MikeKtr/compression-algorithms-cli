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

```bash
# Kompresja pliku tekstowego przy użyciu drzewa Huffmana
cargo run -- file --input shakespiere.txt --output sko.txt --compression-algo huffmanTree

# Konwersja/kompresja pliku BMP do formatu PNG
cargo run -- file --input greenland_grid_vel.bmp --output c_green.png --compression-algo png

# Kompresja pliku audio WAV przy użyciu algorytmu LZW
cargo run -- audio --input audio.wav --output c_sample.lzw --compression-algo lzw
```

## Refleksje nad językiem Rust

### Co ułatwiło implementację?

    - Precyzyjna kontrola nad typami numerycznymi, podział na typy u8, u16, u64 itp.

    - Inferencja typów: Zaawansowane wnioskowanie typów przez kompilator.

### Co stanowiło największe wyzwanie?

    - Rygorystyczny system prawa własności (Ownership) przy operacjach I/O.
