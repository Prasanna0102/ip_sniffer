use bpaf::*;
use std::fs::File;
use std::io::{self, Write};
use std::net::IpAddr;
use std::sync::{Arc, mpsc::{channel, Sender}};
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::task;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    address: IpAddr,
    start_port: u16,
    end_port: u16,
    threads: u16,
    output: String,
    timeout: u64,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub address: IpAddr,
    pub start_port: u16,
    pub end_port: u16,
    pub threads: u16,
    pub output: String,
    pub timeout: u64,
}

fn options() -> OptionParser<Arguments> {
    let address = long("ip")
        .help("IP Address to scan")
        .argument("IP");

    let start_port = long("start")
        .help("Starting port for scanning")
        .argument::<u16>("START_PORT")
        .fallback(1);

    let end_port = long("end")
        .help("Ending port for scanning")
        .argument::<u16>("END_PORT")
        .fallback(1000);

    let threads = long("threads")
        .help("Number of threads to use")
        .argument::<u16>("THREADS")
        .fallback(100);

    let output = long("output")
        .help("Output file for results")
        .argument::<String>("OUTPUT")
        .fallback("result.txt".to_string());

    let timeout = long("timeout")
        .help("Connection timeout in milliseconds")
        .argument::<u64>("TIMEOUT")
        .fallback(1000);

    construct!(Arguments {
        address,
        start_port,
        end_port,
        threads,
        output,
        timeout
    })
    .to_options()
}

async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr, timeout: u64) {
    let timeout = tokio::time::Duration::from_millis(timeout);
    let result = tokio::time::timeout(timeout, TcpStream::connect(format!("{}:{}", addr, port))).await;

    match result {
        Ok(Ok(_)) => {
            tx.send(port).unwrap();
        }
        Err(_) => {
            println!("Port {} scan timed out", port);
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() {
    let opts = options().run();
    let start_time = Instant::now();

    println!("Starting IP Sniffer...");

    let (tx, rx) = channel();
    let pb = ProgressBar::new((opts.end_port - opts.start_port) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
            .expect("Invalid template")
            .progress_chars("#>-"),
    );

    // Create a semaphore to limit the number of concurrent tasks
    let semaphore = Arc::new(Semaphore::new(opts.threads as usize));

    for port in opts.start_port..=opts.end_port {
        let tx = tx.clone();
        let addr = opts.address;
        let timeout = opts.timeout;
        let semaphore = semaphore.clone(); // Clone the semaphore

        task::spawn(async move {
            let permit = semaphore.acquire().await.unwrap();
            scan(tx, port, addr, timeout).await;
            drop(permit); // Release the permit
        });

        pb.inc(1);
    }

    drop(tx);

    let mut open_ports = vec![];
    for port in rx {
        open_ports.push(port);
    }

    pb.finish_with_message("Scan completed.");

    if let Err(e) = save_results(&opts.output, &open_ports) {
        eprintln!("Error saving results: {}", e);
    } else {
        println!("Results saved to {}", opts.output);
    }

    print_summary(&open_ports, start_time);

    println!("\nOpen ports:");
    open_ports.sort();
    for port in open_ports {
        println!("Port {} is open", port);
    }
}

fn save_results(file_path: &str, ports: &[u16]) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    for port in ports {
        writeln!(file, "Port {} is open", port)?;
    }
    Ok(())
}

fn print_summary(open_ports: &[u16], start_time: Instant) {
    let duration = start_time.elapsed();
    let total_ports = open_ports.len();

    println!("\nScan Summary:");
    println!("Total open ports: {}", total_ports);
    println!("Elapsed time: {:?}", duration);

    let summary_file_path = "summary.txt";
    let mut file = File::create(summary_file_path).expect("Unable to create file");
    writeln!(file, "Scan completed.").expect("Unable to write to file");
    writeln!(file, "Total open ports: {}", total_ports).expect("Unable to write to file");
    writeln!(file, "Elapsed time: {:?}", duration).expect("Unable to write to file");
}
