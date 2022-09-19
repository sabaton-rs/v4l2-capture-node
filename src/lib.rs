pub mod v4l2;

use std::{time::Duration, ptr::copy_nonoverlapping};
// Main Library file
use sabaton_mw::{NodeBuilder, error::MiddlewareError, PublishOptions};
use tracing::{debug, error,info, span, Level};
use v4l::io::traits::{CaptureStream, Stream};
use ros_signals::{sensors::image::{Image1080p4BPP, Encoding}, standard::Header, standard::Timestamp};

use crate::v4l2::CaptureStreamer;

pub fn example_node_main() -> Result<(),MiddlewareError> {

    let node =   NodeBuilder::default()
        .with_shared_memory(true)
        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
        .build("v4l2-capture".to_owned()).expect("Node creation error");
    
    let mut shm_publish_options = PublishOptions::default();
    let shm_publish_options =  shm_publish_options.with_durability(sabaton_mw::qos::QosDurability::Volatile)
        .with_reliability(sabaton_mw::qos::QosReliability::Reliable( Duration::from_millis(1000) ))
        .with_history(sabaton_mw::qos::QosHistory::KeepLast(1));

        let mut writer = node.advertise::<Image1080p4BPP>(&shm_publish_options).unwrap();

        let res = node.spin(move || {
        
        span!(target: "MAIN", Level::TRACE, "Application Main Loop");
        info!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::task::spawn_blocking( move || {

            let mut capture_stream = CaptureStreamer::new("/dev/video0").unwrap(); 
            loop {
                let (buf, meta) = capture_stream.next().unwrap();

                if let Ok(mut loaned_image) = writer.loan() {
                    println!("Received buffer: {}", buf.len());
                    println!("{}:{:}",  meta.sequence,meta.timestamp);
                    let image = loaned_image.as_mut_ptr().unwrap();
                    unsafe {
                    let ts = Timestamp {
                        sec : meta.timestamp.sec as u64,
                        nsec : (meta.timestamp.usec*1000) as u32,
                    };
                    (*image).header = Header { seq: meta.sequence, stamp: ts };
                    (*image).set_resolution();
                    (*image).encoding = Encoding::RGBA8;
                    (*image).is_bigendian = false;
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
