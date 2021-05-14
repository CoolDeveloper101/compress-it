use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;
use snap::raw::{max_compress_len as _snappy_max_compress_length, decompress_len as _snappy_decompress_len};
use snap::write::FrameEncoder;
use snap::read::FrameDecoder;
// use flate2::write::GzEncoder;
use flate2::{ Compression, GzBuilder};
use std::io::prelude::*;


// use web_sys::Blob;

// #[wasm_bindgen(start)]
// pub fn run() -> () {
//     // Use `web_sys`'s global `window` function to get a handle on the global
//     // window object.
//     let string = "
//         Lorem ipsum dolor sit amet, consectetur adipisicing elit. Cum est reiciendis pariatur ad possimus nihil magnam iure ullam eveniet ducimus, veniam quos earum ratione eum! Natus nostrum libero nobis! Repellendus illo consequuntur aliquam exercitationem ea tempora eaque doloremque? Sint dolores placeat dolorum eligendi corrupti, reiciendis aspernatur magnam nemo nobis consectetur quisquam, possimus asperiores, eum repellendus recusandae quaerat iste dicta id fugiat vero consequatur! Provident qui modi quibusdam. Sequi officia praesentium qui quibusdam aut ullam nesciunt quod facere, totam voluptatum. Iusto rem, harum ratione temporibus accusamus quod? Ex ducimus vitae obcaecati quidem necessitatibus consectetur ad esse, porro magnam quam sit ut adipisci quasi nostrum, cupiditate libero soluta! Placeat, cum. Laboriosam, ab voluptate facilis quam, accusantium quas consequatur animi, maiores veniam perspiciatis eius commodi neque corrupti earum aut voluptatibus porro quidem. Obcaecati architecto accusantium magnam voluptatem inventore debitis autem nulla, nisi illum officia alias expedita vitae sed ratione vel, maiores blanditiis dolore commodi quam ducimus ex ea! Doloribus accusamus adipisci qui modi et nostrum eum eius deleniti dolore maiores. Quia officiis, consequatur velit assumenda dignissimos quod ratione rem tempore deserunt fuga labore sapiente eius suscipit ipsa aperiam, ea voluptas! Cumque, optio sed enim quae odit quasi pariatur laboriosam beatae quas! Magni repellendus sint sed culpa porro quaerat unde, est voluptatem quia fugiat beatae, cupiditate explicabo ipsum. Voluptatem a assumenda asperiores, ab qui maiores, numquam tempore nemo animi aliquid suscipit nulla quidem odit. Voluptas facere harum ratione ad, quaerat expedita saepe delectus quibusdam libero placeat neque voluptatum blanditiis mollitia, dolorum laudantium iure autem sequi illo, unde numquam quisquam doloribus ex veniam veritatis. Minus sapiente consequatur blanditiis eligendi, molestiae, magnam reiciendis, velit dolor eius ab aliquam ratione assumenda officia tempore molestias! Recusandae aperiam eligendi voluptatibus aut placeat? Facere dolorum illum quaerat sunt temporibus voluptate! Laudantium dicta magni excepturi repellat nemo velit natus sunt. Eum sed nisi hic asperiores, perferendis corporis voluptatem harum beatae perspiciatis corrupti error ducimus nulla ratione aliquam culpa maxime. Distinctio, voluptate optio dicta iure molestiae expedita eaque eum voluptates sunt! Voluptatem asperiores ducimus corrupti voluptates numquam quisquam ullam totam reiciendis, repellat est, ratione accusantium esse praesentium vel eius provident ipsum suscipit necessitatibus possimus placeat distinctio cumque excepturi. Facere illum ex sed doloremque nulla delectus earum voluptates assumenda! Nisi voluptas magni, beatae ipsam doloremque deserunt porro suscipit ad ipsa consequuntur repudiandae similique ab necessitatibus sit adipisci quasi veritatis fugiat, debitis dicta perferendis voluptates id quas temporibus ut. Amet, a. Fuga odio illum commodi qui consequatur similique sint reprehenderit accusamus. Magnam animi dolorum itaque temporibus aspernatur numquam facilis impedit soluta aliquam obcaecati quaerat dolor modi sit dolore maxime, explicabo quos fugiat! Voluptates quaerat ipsam iure quo enim quae saepe, aspernatur maxime repellat? Assumenda laboriosam perferendis debitis commodi veritatis doloremque. Quo fugiat esse veritatis molestias. Itaque adipisci inventore magnam provident! Ipsam omnis eligendi nesciunt porro nulla eaque quos soluta veritatis necessitatibus minima eveniet doloribus minus, vitae, repellat voluptates quia incidunt hic rerum reiciendis blanditiis. Rerum delectus quod ab, veniam nisi totam! Dolor cum saepe expedita doloribus, quos assumenda cupiditate autem velit ea adipisci porro nemo quas repellat odio. Odit accusantium placeat ipsum officiis dicta similique maiores cum nam, impedit, omnis, culpa suscipit nobis nostrum dolorem architecto quia dolor ex sequi consequatur fuga. Quaerat inventore voluptas labore praesentium ipsam, minus dolorum ratione officiis non? Saepe molestiae laboriosam atque tempora, neque eius nam quae voluptatibus error delectus possimus beatae nisi vel accusamus recusandae similique iusto porro quam hic nobis mollitia eaque quod esse voluptate! Autem amet voluptatibus quas voluptate repudiandae. Libero modi minus, exercitationem inventore quos eaque esse reiciendis, recusandae quam enim dolor quisquam officia excepturi id. Quidem quod illo cumque ipsam quo quae? Vitae quia at aut voluptas molestiae accusantium quis obcaecati eaque. Eveniet ab quis enim pariatur, sit voluptatibus facilis laborum unde ipsam eligendi explicabo harum sunt natus fugit non nesciunt quos, error totam expedita culpa tempora. Cupiditate quam repellendus in perferendis itaque deleniti perspiciatis et? Ducimus placeat, facere doloremque labore aliquam eius, hic voluptates, fuga nemo mollitia commodi. Ipsum expedita, soluta eveniet nostrum maiores sit saepe ex perspiciatis fuga, consectetur, explicabo neque tenetur in harum quaerat itaque. Eius iusto necessitatibus distinctio optio, tempora deleniti adipisci? Deleniti expedita dignissimos, perferendis ratione obcaecati consectetur placeat rem quidem repudiandae sit. Suscipit autem quos tempora, cum sapiente voluptate doloribus tempore saepe, ratione omnis dolorem. Fuga assumenda similique totam pariatur voluptatibus porro veritatis ab exercitationem atque animi, impedit odit! Provident, reprehenderit! Deserunt ipsum neque nostrum odit eos nesciunt porro quos itaque nemo? Velit est, nam optio sit deleniti perferendis porro eius exercitationem iste pariatur accusamus ea accusantium consequatur inventore, assumenda vero eligendi, iusto corporis non! Rem delectus amet atque debitis suscipit aperiam culpa, deleniti exercitationem quis vero reprehenderit consequatur optio repellat harum, mollitia minus ipsam omnis ipsa? Harum natus cum ex a numquam expedita, amet praesentium quisquam porro perspiciatis impedit, sapiente voluptatibus veniam doloribus temporibus mollitia molestiae et perferendis sint! Nemo maxime fugit ab natus tempore quidem similique quibusdam, molestias rem perspiciatis enim doloremque amet quis corrupti dolore facere asperiores inventore? Esse beatae iusto architecto delectus tenetur quae. Corrupti repellat id, ea ipsa ipsam ratione dolor tempore voluptas porro maxime dignissimos odit eaque debitis culpa molestiae quo laboriosam, perspiciatis exercitationem quia nulla tempora, adipisci dolores omnis autem. Illo numquam quo voluptate deserunt quis sequi distinctio assumenda maxime at earum cum aut dignissimos odio soluta officia illum iure in, recusandae ut reprehenderit. Dolorum temporibus obcaecati ab. Voluptate, eaque libero facilis quo inventore voluptas. Magnam, aut natus dicta itaque impedit aperiam iure eos autem optio assumenda suscipit maiores tempora corporis sunt exercitationem dolore necessitatibus? Quia tempore possimus tenetur id incidunt perferendis alias ea nulla, esse cum, magni deserunt quis quidem corporis consequatur officiis quisquam laboriosam dolore quo! Iusto non, tempore animi ab saepe aut voluptates consectetur, dolores reiciendis cum mollitia obcaecati delectus aperiam repellat beatae sit nulla voluptas tenetur perferendis laborum iste! Sequi iste voluptas voluptate eius ad quos laborum beatae amet. Adipisci, libero nihil, saepe et doloremque hic harum beatae laborum minima, ipsam amet vitae dicta. Quaerat alias distinctio sequi tempora voluptas quidem quisquam a natus recusandae, sapiente porro.
//     ";
//     let compressed_vec = compress(string.as_bytes());
//     let a = Blob::new_with_u8_array_sequence(JsValue::from);
//     let compressed: &[u8] = &compressed_vec;
//     log_uint8array(compressed);
    
// }

// #[wasm_bindgen]
// extern {
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_str(s: &str);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_uint8array(arr: &[u8]);
// }

#[wasm_bindgen]
pub fn snappy_compress(input: &[u8]) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(_snappy_max_compress_length(input.len()));
    {
        let mut encoder = FrameEncoder::new(&mut buffer);
        encoder.write_all(input).expect_throw("Failed to compress file.");
        encoder.flush().expect_throw("Failed to do something!");
    }
    buffer
}

#[wasm_bindgen]
pub fn snappy_decompress(input: &[u8]) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(_snappy_decompress_len(input).expect_throw("The compressed file is invalid!"));
    {
        let mut encoder = FrameDecoder::new(input);
        encoder.read_exact(&mut buffer).expect_throw("Failed to compress file.");
    }
    buffer
}

#[wasm_bindgen]
pub fn snappy_max_compress_len(input_len: usize) -> usize  {
    match _snappy_max_compress_length(input_len) {
        0 => throw_str("The length of the uncompressed buffer is larger than the maximum allowed size."),
        res => res
    }
}

#[wasm_bindgen]
pub fn snappy_decompress_len(input: &[u8]) -> usize {
    _snappy_decompress_len(input).expect_throw("The input provided is invalid!")
}

#[wasm_bindgen]
pub fn gzip_compress(filename: &str, input: &[u8]) -> Vec<u8> {
    // let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    // encoder.write_all(input).expect_throw("Could not compress the file!");
    // encoder.finish().unwrap_throw()
    let mut gz = GzBuilder::new()
                .filename(filename)
                .write(Vec::new(), Compression::default());
    gz.write(input).expect_throw("Could not compress");
    gz.finish().unwrap_throw()
}






// #[wasm_bindgen]
// pub fn max_compressed_length(num: usize) -> usize {
// 	unsafe {
// 		snappy_max_compressed_length(num)
// 	}
// }

// #[wasm_bindgen]
// pub fn validate_compressed_buffer(src: &[u8]) -> bool {
// 	unsafe {
// 		snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0
// 	}
// }

// #[wasm_bindgen]
// pub fn compress(src: &[u8]) -> Vec<u8> {
// 	unsafe {
// 		let srclen = src.len() as size_t;
// 		let psrc = src.as_ptr();

// 		let mut dstlen = snappy_max_compressed_length(srclen);
// 		let mut dst = Vec::with_capacity(dstlen as usize);
// 		let pdst = dst.as_mut_ptr();

// 		snappy_compress(psrc, srclen, pdst, &mut dstlen);
// 		dst.set_len(dstlen as usize);
// 		dst
// 	}
// }



// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }