use clap::Parser;
use legion_npx::{LegionCLI, NpxError};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), NpxError> {
    LegionCLI::parse().run().await
}
