#![forbid(unsafe_code)]

pub mod v4l2;

use std::time::Duration;
// Main Library file
use sabaton_mw::{NodeBuilder, error::MiddlewareError};
use tracing::{debug, info, span, Level};
use v4l::io::traits::{CaptureStream, Stream};

use crate::v4l2::CaptureStreamer;

pub fn example_node_main() -> Result<(),MiddlewareError> {

    let node =   NodeBuilder::default()
        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
        .build("example-node".to_owned()).expect("Node creation error");

        let res = node.spin(|| {
        
        span!(target: "MAIN", Level::TRACE, "Application Main Loop");
        info!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::task::spawn_blocking(  || {

            let mut capture_stream = CaptureStreamer::new("/dev/video0").unwrap(); 
            loop {
                let (buf, meta) = capture_stream.next().unwrap();

                println!("Received buffer: {}", buf.len());
                println!("{}:{:}",  meta.sequence,meta.timestamp);
            }

         });
         
    });


    res

}
