use minifb::{Key, Window, WindowOptions, Scale, ScaleMode};
use cosmic_text::{Attrs, Color, FontSystem, SwashCache, Buffer, Metrics};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Fractal - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    test_cosmic(&mut buffer, WIDTH, HEIGHT);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn test_cosmic(output: &mut [u32], width: usize, height: usize) {
    // A FontSystem provides access to detected system fonts, create one per application
    let font_system = FontSystem::new();

    // A SwashCache stores rasterized glyphs, create one per application
    let mut swash_cache = SwashCache::new(&font_system);

    // Text metrics indicate the font size and line height of a buffer
    let metrics = Metrics::new(24, 40);

    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    let mut buffer = Buffer::new(&font_system, metrics);

    // Set a size for the text buffer, in pixels
    buffer.set_size(180, 125);

    // Attributes indicate what font to choose
    let attrs = Attrs::new();

    // Add some text!
    buffer.set_text("Some text 1234 abcdef 伯母さん\n", attrs);

    // Perform shaping as desired
    buffer.shape_until_scroll();

    // Inspect the output runs
    /*
    for run in buffer.layout_runs() {
        for glyph in run.glyphs.iter() {
            println!("{:#?}", glyph);
        }
    }
    */

    // Create a default text color
    let text_color = Color::rgb(0xFF, 0xFF, 0xFF);

    // Draw the buffer (for performance, instead use SwashCache directly)
    buffer.draw(&mut swash_cache, text_color, |x, y, _w, _h, color| {
        let c = color.0 >> 24;
        let c = (c << 16) | (c << 8) | c;
        if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
            return;
        }

        output[(y as usize * width + x as usize) as usize] = c;
    });
}

