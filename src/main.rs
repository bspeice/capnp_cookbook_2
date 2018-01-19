extern crate capnp;

pub mod point_capnp {
    include!(concat!(env!("OUT_DIR"), "/point_capnp.rs"));
}

fn main() {

    let mut builder = capnp::message::Builder::new_default();

    {
        let mut point_msg = builder.init_root::<point_capnp::point::Builder>();

        point_msg.set_x(12);

        point_msg.set_y(14);
    }

    let mut buffer = Vec::new();

    capnp::serialize::write_message(&mut buffer, &builder).unwrap();

    let deserialized = capnp::serialize::read_message(
        &mut buffer.as_slice(),
        capnp::message::ReaderOptions::new()
    ).unwrap();

    let point_reader: capnp::message::TypedReader<capnp::serialize::OwnedSegments, point_capnp::point::Owned> =
		capnp::message::TypedReader::new(deserialized);

	// Because the point_reader is now working with OwnedSegments (which are owned vectors) and an Owned message
	// (which is 'static lifetime), this is now safe
    let handle = std::thread::spawn(move || {

        // The point_reader owns its data, and we use .get() to retrieve the actual point_capnp::point::Reader
		// object from it
		let point_root = point_reader.get().unwrap();

        assert_eq!(point_root.get_x(), 12);

        assert_eq!(point_root.get_y(), 14);
    });

    handle.join().unwrap();
}
