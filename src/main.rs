use colored::*;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use dice6000::api;
use dice6000::config::Config;
use dice6000::game::start_game;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments to find config path
    let mut config_path = "config.yaml".to_string(); // default path
    let mut command_args = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--config" => {
                if i + 1 < args.len() {
                    config_path = args[i + 1].clone();
                    i += 2; // skip both the flag and the value
                } else {
                    println!(
                        "{}",
                        "Error: --config option requires a path argument"
                            .bold()
                            .red()
                    );
                    print_usage(&args[0]);
                    return;
                }
            }
            _ => {
                command_args.push(args[i].clone());
                i += 1;
            }
        }
    }

    // Load configuration with the specified path
    let config = Config::load(&config_path).unwrap_or_else(|e| {
        println!(
            "Warning: Could not load {} ({}), using defaults",
            config_path, e
        );
        Config::init()
    });

    let openai_key = env::var("OPENAI_API_KEY").is_ok();
    let anthropic_key = env::var("ANTHROPIC_API_KEY").is_ok();

    // if !openai_key && !anthropic_key {
    //     println!(
    //         "{}",
    //         "Error: Neither OPENAI_API_KEY nor ANTHROPIC_API_KEY environment variables are set."
    //             .bold()
    //             .red()
    //     );
    //     println!(
    //         "{}",
    //         "Please set at least one API key before running the game:"
    //             .bold()
    //             .yellow()
    //     );
    //     println!("  export OPENAI_API_KEY=your_openai_key");
    //     println!("  export ANTHROPIC_API_KEY=your_anthropic_key");
    //     return;
    // }

    if command_args.is_empty() {
        print_usage(&args[0]);
        return;
    }

    match args[1].as_str() {
        "rules" => {
            display_rules();
        }
        "play" => {
            run_local_game(openai_key, anthropic_key, &config);
        }
        "serve" => {
            run_server_async(&config);
        }
        _ => {
            println!("Unknown command: {}", command_args[0]);
            print_usage(&args[0]);
        }
    }
}

fn run_local_game(openai_key: bool, anthropic_key: bool, config: &Config) {
    println!("Starting local CLI game...");
    println!("Welcome to the 6000 Dice Game!");
    start_game(openai_key, anthropic_key, config);
}

#[tokio::main]
async fn run_server_async(config: &Config) {
    run_api_server(config).await;
}

async fn run_api_server(config: &Config) {
    let app = api::create_router();

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let socket_addr: SocketAddr = addr.parse().expect("Invalid IP or port in config");

    println!("Starting API server on http://{}", socket_addr);
    let listener = TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn print_usage(program_name: &str) {
    println!("Usage: {} [OPTIONS] <command>", program_name);
    println!();
    println!("Options:");
    println!("  -c, --config <path>    Path to configuration file (default: config.yaml)");
    println!();
    println!("Commands:");
    println!("  rules                  Display the game rules");
    println!("  play                   Play the local game");
    println!("  serve                 Start server");
    println!();
    println!("Examples:");
    println!("  {} play", program_name);
    println!("  {} --config my_config.yaml play", program_name);
    println!("  {} -c /path/to/config.yaml rules", program_name);
}

fn display_rules() {
    println!("{}", "SIX THOUSAND DICE GAME RULES".bold().green());
    println!("{}", "============================".green());
    println!();
    println!("OBJECTIVE:");
    println!("  Be the first player to reach or exceed 6,000 points.");
    println!();
    println!("GAMEPLAY:");
    println!("  1. Players take turns rolling six dice");
    println!("  2. After each roll, you must set aside at least one scoring die");
    println!("  3. You may continue rolling with remaining dice or bank your points");
    println!("  4. If you cannot score with a roll, you lose all points from that turn");
    println!();
    println!("SCORING COMBINATIONS:");
    println!("  - Straight (1-2-3-4-5-6): 2000 points, all dice used");
    println!("  - Three pairs: 1500 points, all dice used");
    println!("  - Six of a kind: face value × 1000 points");
    println!("  - Three 1s: 1000 points");
    println!("  - Three of 2-6: face value × 100 points");
    println!("  - Single 1: 100 points each");
    println!("  - Single 5: 50 points each");
}
