pub mod extract_information;
pub mod get_brief;
pub mod get_document;
pub mod html_to_markdown;
pub mod render_markdown;

use anyhow::Result;
use clap::Parser;

use extract_information::*;
use get_brief::*;
use get_document::*;
use html_to_markdown::*;
use render_markdown::*;

async fn print_full(status_code: u16) -> Result<()> {
    let document = get_document(status_code).await?;
    let information = extract_infomation(document)?;
    let markdown = html_to_markdown(information)?;
    render_markdown(markdown).await
}

fn print_brief(status_code: u16) -> Result<()> {
    let brief = get_brief(status_code)?;
    println!("{}", brief);

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    status_code: u16,

    #[arg(short, long, default_value_t = false)]
    brief: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.brief {
        print_brief(args.status_code)
    } else {
        print_full(args.status_code).await
    }
}
