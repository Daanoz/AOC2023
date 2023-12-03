use days::PuzzleViewport;
use eframe::egui;

mod days;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

#[derive(Default)]
struct MyApp {
    viewport_puzzle: Option<Box<dyn days::PuzzleViewportUi>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AOC 2023");
            ui.horizontal_wrapped(|ui| {
                for day in 1..=25 {
                    let caption = format!("Day {:02}", day);
                    if ui.button(caption).clicked() {
                        let solution = aoc2023::days::get_day(day).unwrap();
                        self.viewport_puzzle = Some(Box::new(PuzzleViewport::new(day, solution)));
                    }
                }
            })
        });

        if self.viewport_puzzle.is_some() {
            let viewport_puzzle = self.viewport_puzzle.as_ref().unwrap();
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("puzzle_viewport"),
                egui::ViewportBuilder::default()
                    .with_title(format!(
                        "AOC 2023 Puzzle Day {:02}",
                        viewport_puzzle.get_day()
                    ))
                    .with_inner_size([400.0, 300.0]),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );
                    self.viewport_puzzle.as_mut().unwrap().update(ctx);
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent to close us.
                        self.viewport_puzzle = None;
                    }
                },
            );
        }
    }
}
