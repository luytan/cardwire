mod args;
mod dbus;
mod output;
use args::{Args, CliMode, Commands};
use clap::Parser;
use dbus::DaemonClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let connection: zbus::Connection = zbus::connection::Builder::system()?.build().await?;
    let client: DaemonClient<'_> = DaemonClient::connect(&connection).await?;

    match args.command {
        Commands::Set { mode } => {
            let mode_string = match mode {
                CliMode::Integrated => "integrated".to_string(),
                CliMode::Hybrid => "hybrid".to_string(),
                CliMode::Manual => "manual".to_string(),
            };

            //let response = client.set_mode(mode).await;
            match client.set_mode(mode_string).await {
                Ok(response) => println!("{}", response),
                Err(zbus::Error::MethodError(name, description, _)) => {
                    eprintln!("{}", description.unwrap_or_else(|| name.to_string()))
                }
                Err(e) => eprintln!("Error: {e:?}"),
            };
        }
        Commands::Get => {
            let response_result = client.get_mode().await;
            match response_result {
                Ok(response) => eprintln!("{}", response),
                Err(e) => eprintln!("Error: {e:?}"),
            };
        }
        Commands::List { full: _, json: _ } => {
            let mut response: Vec<(u32, String, String, String, bool, bool)> =
                client.list_gpus().await?;
            response.sort_by_key(|row| row.0);
            output::print_gpu_table(&response);
        }
        Commands::Gpu { id, action } => {
            match client.set_gpu_block(id, action.block).await {
                Ok(response) => println!("{}", response),
                Err(zbus::Error::MethodError(name, description, _)) => {
                    eprintln!("Error: {}", description.unwrap_or_else(|| name.to_string()))
                }
                Err(e) => eprintln!("Error: {e:?}"),
            };
        }
    }

    Ok(())
}
