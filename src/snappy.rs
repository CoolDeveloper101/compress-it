type size_t = usize;
type c_int = i32;

#[link(name="snappy", kind="static")]
extern {
	pub(crate) fn snappy_max_compressed_length(source_length: size_t) -> size_t;
	pub(crate) fn snappy_compress(input: *const u8,
					input_length: size_t,
					compressed: *mut u8,
					compressed_length: *mut size_t) -> c_int;
	pub(crate) fn snappy_uncompress(compressed: *const u8,
						compressed_length: size_t,
						uncompressed: *mut u8,
						uncompressed_length: *mut size_t) -> c_int;
	pub(crate) fn snappy_uncompressed_length(compressed: *const u8,
								compressed_length: size_t,
								result: *mut size_t) -> c_int;
	pub(crate) fn snappy_validate_compressed_buffer(compressed: *const u8,
										compressed_length: size_t) -> c_int;
}
