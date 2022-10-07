pub mod v4l2;

use std::{ptr::copy_nonoverlapping, time::Duration};
// Main Library file
use robotics_signals::{
    sensors::image::{Encoding, Image1080p4BPP},
    standard::Header,
    standard::Timestamp,
};
use sabaton_mw::{error::MiddlewareError, NodeBuilder, PublishOptions};
use tracing::{debug, error, info, span, Level};
use v4l::io::traits::{CaptureStream, Stream};

use crate::v4l2::CaptureStreamer;

pub struct Params<'a> {
    pub maybe_group: Option<&'a str>,
    pub maybe_instance: Option<&'a str>,
    pub video_dev: String,
}

pub fn example_node_main(params: &Params) -> Result<(), MiddlewareError> {
    let video_dev = params.video_dev.to_owned();

    let mut node = NodeBuilder::default().with_shared_memory(true);
    //.multi_threaded()  Enable this if you want a multi-threaded runtime
    //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
    if let Some(group) = params.maybe_group {
        node = node.with_group(group.into());
    }
    if let Some(instance) = params.maybe_instance {
        node = node.with_instance(instance.into());
    }
    let node = node
        .build("v4l2-capture".to_owned())
        .expect("Node creation error");

    let mut shm_publish_options = PublishOptions::default();
    let shm_publish_options = shm_publish_options
        .with_durability(sabaton_mw::qos::QosDurability::Volatile)
        .with_reliability(sabaton_mw::qos::QosReliability::Reliable(
            Duration::from_millis(1000),
        ))
        .with_history(sabaton_mw::qos::QosHistory::KeepLast(1));

    let mut writer = node
        .advertise::<Image1080p4BPP>(&shm_publish_options)
        .unwrap();

    let res = node.spin(move || {
        span!(target: "MAIN", Level::TRACE, "v4l2-capture Main Loop");
        info!("Application Main Loop Started with tick interval 100mS");
        
        println!(
            "Size of image = {:?}",
            std::mem::size_of::<Image1080p4BPP>()
        );

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::task::spawn_blocking(move || {
            let mut capture_stream = CaptureStreamer::new(&video_dev).unwrap();
            loop {
                let (buf, meta) = capture_stream.next().unwrap();

                if let Ok(mut loaned_image) = writer.loan() {
                    println!("Received buffer: {}", buf.len());
                    println!("{}:{:}", meta.sequence, meta.timestamp);
                    let image = loaned_image.as_mut_ptr().unwrap();
                    unsafe {
                        let ts = Timestamp {
                            sec: meta.timestamp.sec as u64,
                            nsec: (meta.timestamp.usec * 1000) as u32,
                        };
                        (*image).header = Header {
                            seq: meta.sequence,
                            stamp: ts,
                        };
                        (*image).set_resolution();
                        (*image).encoding = Encoding::RGBA8;
                        copy_nonoverlapping(buf.as_ptr(), (*image).data.as_mut_ptr(), buf.len());
                    }

                    let finalized_image = loaned_image.assume_init();
                    writer.return_loan(finalized_image).unwrap();
                } else {
                    error!("Unable to get loan");
                }
            }
        });
    });

    res
}
