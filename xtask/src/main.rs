use structopt::StructOpt;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

/// Tooling for `yosh.is`
#[derive(StructOpt)]
enum Opt {
    /// Generate source code from static sources
    Generate,
}

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Generate => {
            let path = std::env::current_dir()?.join("../../webidls");
            html_bindgen::generate_html(&path)
        }
    }
}
