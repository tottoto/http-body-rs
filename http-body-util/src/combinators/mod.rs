//! Combinators for the `Body` trait.

mod box_body;
mod collect;
mod frame;
mod fuse;
mod map_err;
mod map_frame;
mod with_trailers;

pub use self::{
    box_body::{BoxBody, UnsyncBoxBody},
    collect::Collect,
    frame::Frame,
    fuse::Fuse,
    map_err::MapErr,
    map_frame::MapFrame,
    with_trailers::WithTrailers,
};
