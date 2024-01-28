use clap::Parser;
use legion_new::{LegionNew, NpxError};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), NpxError> {
    LegionNew::parse().run().await
}
