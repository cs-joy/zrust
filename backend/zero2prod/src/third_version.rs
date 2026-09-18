// crate
use zero2prod::run;

// runtime
#[tokio::main]
async fn main() -> std::io::Result<()> /* future trait */ {
    run().await
}
