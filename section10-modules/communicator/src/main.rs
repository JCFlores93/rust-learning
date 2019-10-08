extern crate communicator;
use a::series::of::nested_module;
use TrafficLight::{Red, Green};
use TrafficLight::*;

enum TrafficLight{
    Red, Yellow, Green
}

fn main() {
    communicator::client::connect();
    a::series::of::nested_module();
    nested_module();
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_module(){

            }
        }
    }
}
