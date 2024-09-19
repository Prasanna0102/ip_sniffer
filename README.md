<<<<<<< HEAD

# IP Sniffer

**IP Sniffer** is a high-performance, multi-threaded port scanner written in Rust. It allows you to scan a range of ports on a specified IP address, providing real-time progress updates and detailed scan summaries.

## Features

- Multi-threaded scanning for faster results
- Configurable timeout for connections
- Real-time progress bar with estimated time remaining
- Detailed scan summary including elapsed time and total open ports
- Output results to a file

## Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/Prasanna0102/ip_sniffer.git
    cd ip_sniffer
    ```

2. **Build the project:**

    ```bash
    cargo build --release
    ```

## Usage

Run the IP Sniffer with the following command:

    ```bash
    cargo run -- --ip=IP_ADDRESS --start=START_PORT --end=END_PORT --threads=NUMBER_OF_THREADS --output=RESULT_FILE --timeout=TIMEOUT
    ```

### Options

- `--ip`: IP Address to scan
- `--start`: Starting port for scanning (default: 1)
- `--end`: Ending port for scanning (default: 1000)
- `--threads`: Number of threads to use (default: 100)
- `--output`: Output file for results (default: result.txt)
- `--timeout`: Connection timeout in milliseconds (default: 1000)

### Example

    ```bash
    cargo run -- --ip=192.168.1.1 --start=1 --end=65535 --threads=100 --output=open_ports.txt --timeout=500
    ```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Tokio](https://tokio.rs) for asynchronous runtime
- [Indicatif](https://docs.rs/indicatif/latest/indicatif/) for progress bar
- [Serde](https://serde.rs) for serialization/deserialization
=======
# ip_sniffer
IP Sniffer: A multi-threaded port scanner built in Rust. Scans a range of ports on a specified IP address with configurable thread count, timeout, and output options. Includes real-time progress tracking and detailed scan summary.
>>>>>>> origin/main
