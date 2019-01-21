#![feature(uniform_paths)] 
// extern crate rosrust;
use rosrust;
use nalgebra;
#[macro_use]
extern crate rosrust_codegen;
#[macro_use]
extern crate approx;


mod tf_buffer;
mod msg;
mod interpolation;
mod transform_storage;
mod time_cache;
mod time_cache_interface;

use tf_buffer::tf::FrameId;

rosmsg_include!(std_msgs/Header);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_frame_name() {
        FrameId::new("map");
    }



}
