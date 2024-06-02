use eframe::egui;
use egui::{ColorImage, TextureHandle};

mod error;
mod font;
mod http;
mod renderer;

#[derive(Default)]
pub struct MyApp {
    texture: Option<TextureHandle>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.texture.is_none() {
            // FIXME: Handle window resizing. Texture should be redrawn whenever the window size changes.
            self.init_texture(ctx)
        }

        let texture_ref = self.texture.as_ref().unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(texture_ref);
        });
    }
}

impl MyApp {
    fn init_texture(&mut self, ctx: &egui::Context) {
        let window_width = ctx.available_rect().width() as usize - 18;
        let window_height = ctx.available_rect().height() as usize - 18;
        let mut bitmap = draw_pixels(window_width, window_height);

        renderer::render_text(&mut bitmap, "a", 50, 50, window_width, 2).unwrap();

        // FIXME: Implement font rendering
        let parsed_font = font::parse_from_file("assets/fonts/arial.ttf");
        match parsed_font {
            Ok(f) => println!("parsed_font {:#?}", f),
            Err(e) => println!("failed to parse font at filepath 'invalid_filepath': {}", e),
        }

        let color_image: ColorImage =
            ColorImage::from_rgba_unmultiplied([window_width, window_height], &bitmap);
        self.texture = Some(ctx.load_texture("bitmap", color_image, Default::default()));
    }
}

fn draw_pixels(width: usize, height: usize) -> Vec<u8> {
    let mut pixels = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let offset = (y * width + x) * 4;
            pixels[offset] = 255; // Red
            pixels[offset + 1] = 255; // Green
            pixels[offset + 2] = 255; // Blue
            pixels[offset + 3] = 255; // Alpha
        }
    }
    pixels
}

fn main() {
    eframe::run_native(
        "CapyNet",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    )
    .unwrap();
}
