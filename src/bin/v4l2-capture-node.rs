#![forbid(unsafe_code)]
use clap::{arg, command};
use v4l2_capture_node::example_node_main;
use tracing::Level;
fn main() {
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom configuration file for v4l2-capture-node"
            )
            .required(false)
            .allow_invalid_utf8(false),
        )
        .arg(
            arg!(
                -d --debug ... "Turn on debugging"
            )
            .required(false)
        )
        
        .arg(
            arg!(
                -f --debugfilter "List of debug spans to filter on. The default is to show all spans"
            )
            .required(false)
        )
        .get_matches();

    let trace_filter = match matches.occurrences_of("debug") {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        _ => Level::TRACE
    };

    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(trace_filter)
        // sets this to be the default, global collector for this application.
        .with_target(true)
        .init();


    /*
        Process the command line arguments here and create a configuration structure that is then passed into the main function.
     */
    

    example_node_main().expect("Error running main");
}
