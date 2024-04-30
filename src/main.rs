fn main() {
    // This works
    // tracing_subscriber::fmt()
    //     .json()
    //     .init();

    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().json())
        .init();

    let span = tracing::span!(tracing::Level::INFO, "a span");

    let _enter = span.enter();
    tracing::info!("Hello World!");
}