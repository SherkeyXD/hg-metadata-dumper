# HG Metadata Dumper

Dumps `global-metadata.dat` for hg games, written in rust.

## Usage

```bash
Usage: hg-metadata-dumper [OPTIONS] [INPUT] [OUTPUT]

Arguments:
  [INPUT]     Input PE file path (default: GameAssembly.dll)
  [OUTPUT]    Output decrypted file path (default: global-metadata.dat)

Options:
  -v, --verbose  Show detailed metadata info
  -h, --help  Show this help message
```
