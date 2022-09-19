use std::io;
use std::path::Path;
use std::time::Instant;

use v4l::buffer::{Type, Metadata};
use v4l::io::traits::CaptureStream;
use v4l::video::capture::Parameters;
use v4l::video::Capture;
use v4l::{prelude::*, Fraction};

pub struct CaptureStreamer(UserptrStream);

impl CaptureStreamer {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        println!("Using device: {}\n", path);
        // Capture 4 frames by default
        let count = 4;

        // Allocate 4 buffers by default
        let buffer_count = 4;

        let dev = Device::with_path(path)?;
        let format = dev.format()?;
        let params = dev.params()?;
        println!("Active format:\n{}", format);
        println!("Active parameters:\n{}", params);

        let params = dev
            .set_params(&Parameters::new(Fraction::new(1, 60)))
            .unwrap();

        println!("Active parameters after change:\n{}", params);

        // Setup a buffer stream and grab a frame, then print its data
        let mut stream = UserptrStream::with_buffers(&dev, Type::VideoCapture, buffer_count)?;

        // warmup
        stream.next()?;

        Ok(Self(stream))
    }

    pub fn next<'a>(&'a mut self) -> io::Result<(& [u8], &'a Metadata)> {
        self.0.next()

    }

}

