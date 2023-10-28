use tracing_subscriber::EnvFilter;

pub fn initialize_logger_global(verbosity: u8) {
    match verbosity {
        0 => std::env::set_var("RUST_LOG", "info"),
        1 => std::env::set_var("RUST_LOG", "debug"),
        2 | 3 => std::env::set_var("RUST_LOG", "trace"),
        _ => std::env::set_var("RUST_LOG", "trace"),
    };

    let format = tracing_subscriber::fmt::format()
        .with_target(true)
        .with_level(verbosity == 3 || verbosity == 1);
    //.with_line_number(verbosity == 3 || verbosity == 1)
    //.with_source_location(verbosity == 3 || verbosity == 1);

    // Filter out undesirable logs.
    let filter = EnvFilter::from_default_env()
        .add_directive("mio=off".parse().unwrap())
        .add_directive("tokio_util=off".parse().unwrap())
        .add_directive("anemo=off".parse().unwrap())
        .add_directive("rustls=off".parse().unwrap())
        .add_directive("quinn-proto=off".parse().unwrap())
        .add_directive("quinn=off".parse().unwrap())
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("anfs-rpc=off".parse().unwrap())
        .add_directive("anfs-network=off".parse().unwrap())
        // .add_directive("hyper::proto::h1::conn=off".parse().unwrap())
        // .add_directive("hyper::proto::h1::decode=off".parse().unwrap())
        // .add_directive("hyper::proto::h1::io=off".parse().unwrap())
        // .add_directive("hyper::proto::h1::role=off".parse().unwrap())
        .add_directive("jsonrpsee=off".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(true)
        .event_format(format)
        .init();
}
