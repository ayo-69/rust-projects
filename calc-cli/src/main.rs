use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "calc")]
#[command(about = "A simple CLI calculator app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(value_name = "num_a")]
        num_a: i32,
        #[arg(value_name = "num_b")]
        num_b: i32,
    },
    Sub {
        #[arg(value_name = "num_a")]
        num_a: i32,
        #[arg(value_name = "num_b")]
        num_b: i32,
    },
    Mul {
        #[arg(value_name = "num_a")]
        num_a: i32,
        #[arg(value_name = "num_b")]
        num_b: i32,
    },
    Div {
        #[arg(value_name = "num_a")]
        num_a: f32,
        #[arg(value_name = "num_b")]
        num_b: f32,
    },
    Fact {
        #[arg(value_name = "num")]
        num: u64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { num_a, num_b } => {
            let result = num_a + num_b;
            println!("{} + {} = {}", num_a, num_b, result);
        }
        Commands::Sub { num_a, num_b } => {
            let result = num_a - num_b;
            println!("{} - {} = {}", num_a, num_b, result);
        }
        Commands::Mul { num_a, num_b } => {
            let result = num_a + num_b;
            println!("{} * {} = {}", num_a, num_b, result);
        }
        Commands::Div { num_a, num_b } => {
            let result = num_a / num_b;
            println!("{} / {} = {}", num_a, num_b, result);
        }
        Commands::Fact { num } => {
            if num > 20 {
                println!("number is too large.");
                return;
            }
            let result = fact(num);
            println!("Factorial of {} = {}", num, result);
        }
    }
}

fn fact(n: u64) -> u64 {
    if n == 1 || n == 0 {
        return 1;
    }

    n * fact(n - 1)
}
