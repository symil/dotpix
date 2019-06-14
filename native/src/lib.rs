#[macro_use]
extern crate neon;
extern crate scrap;

use neon::prelude::*;
use scrap::{Capturer, Display};
use std::time::Duration;
use std::thread;
use std::io::ErrorKind::WouldBlock;

fn screenshot(mut cx: FunctionContext) -> JsResult<JsObject> {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (width, height) = (capturer.width(), capturer.height());

    loop {
        // Wait until there's a frame.

        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        let js_width = cx.number(width as f64);
        let js_height = cx.number(height as f64);
        let js_data = JsBuffer::new(&mut cx, buffer.len() as u32).unwrap();

        cx.borrow(&js_data, |data| {
            let slice = data.as_mut_slice::<u8>();

            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) * 4;
                    let r = buffer[i + 2];
                    let g = buffer[i + 1];
                    let b = buffer[i];
                    let a = 255;

                    slice[i] = r;
                    slice[i + 1] = g;
                    slice[i + 2] = b;
                    slice[i + 3] = a;
                }
            }
        });

        let result = JsObject::new(&mut cx);
        result.set(&mut cx, "width", js_width).unwrap();
        result.set(&mut cx, "height", js_height).unwrap();
        result.set(&mut cx, "data", js_data).unwrap();

        return Ok(result);
    }
}

register_module!(mut cx, {
    cx.export_function("screenshot", screenshot)
});
