<h1 align="center">Mandrake</h1>

<p align="center">Harmless annoying noises generator.</p>

## Downloads

|                                               **Windows**                                               |                                             **Linux**                                             |                                             **MacOS**                                             |
|:-------------------------------------------------------------------------------------------------------:|:-------------------------------------------------------------------------------------------------:|:-------------------------------------------------------------------------------------------------:|
| [\>> Download <<](https://github.com/NickSpyker/mandrake/releases/latest/download/mandrake-windows.exe) | [\>> Download <<](https://github.com/NickSpyker/mandrake/releases/latest/download/mandrake-linux) | [\>> Download <<](https://github.com/NickSpyker/mandrake/releases/latest/download/mandrake-macos) |

## Usage

### Run with infinite duration (default)

```bash
mandrake
```

The application will run indefinitely, generating random annoying noises.
To stop it, you must terminate the process through your system's task manager.

### Run with specific duration

```bash
# Run for 10 seconds
mandrake --duration 10
mandrake -d 10
```

The application will automatically stop after the specified duration.

### Command-line options

```
Harmless annoying noises generator

Usage: mandrake.exe [OPTIONS]

Options:
  -d, --duration <DURATION>  Scream duration in seconds
  -h, --help                 Print help (see more with '--help')
  -V, --version              Print version
```

### Platform-specific behavior

- **Windows**: Runs as a background process (no console window in release builds).
- **Unix/Linux/macOS**: Forks into a daemon process.

## Platform-Specific Dependencies

### **Linux**: Requires ALSA development libraries.

```bash
sudo apt-get update && sudo apt-get install -y libasound2-dev
```

## License

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
