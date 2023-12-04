use eframe::egui::{self, Sense, Ui};
use lazy_async_promise::ImmediateValuePromise;
use std::{error::Error, fmt::Display, sync::Arc, time::Duration};

use aoc2023::days::Solution;
use common::Answer;
use futures::lock::Mutex;

pub trait PuzzleViewportUi {
    fn get_day(&self) -> u8;
    fn update(&mut self, ctx: &egui::Context);
}

type PuzzleAnswerPromise = ImmediateValuePromise<(Answer, Duration)>;
type PuzzleShapesPromise = ImmediateValuePromise<Vec<ui_support::Shape>>;

#[derive(Debug)]
struct PuzzleError(String);
impl Display for PuzzleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PuzzleError: {}", self.0)
    }
}
impl Error for PuzzleError {}

pub struct PuzzleViewport {
    pub day: u8,
    puzzle: Arc<Mutex<Box<dyn Solution + Send>>>,
    update_callback_ctx: Option<egui::Context>,
    part_a: Option<PuzzleAnswerPromise>,
    part_b: Option<PuzzleAnswerPromise>,

    visualization_zoom: f32,
    visualization_offset: emath::Vec2,
    shapes: Option<PuzzleShapesPromise>,
}
impl PuzzleViewport {
    pub fn new(day: u8, puzzle: Box<dyn Solution + Send>) -> Self {
        Self {
            day,
            puzzle: Arc::new(Mutex::new(puzzle)),
            update_callback_ctx: None,
            part_a: None,
            part_b: None,

            visualization_zoom: 100.0,
            visualization_offset: emath::Vec2::ZERO,
            shapes: None,
        }
    }
}

impl PuzzleViewport {
    fn update_callback(&self) -> impl Fn() {
        let ctx = self.update_callback_ctx.clone().unwrap();
        move || {
            ctx.request_repaint();
        }
    }

    fn solve_part(&mut self, second_part: bool) -> PuzzleAnswerPromise {
        let day = self.day;
        let update_callback = self.update_callback();
        let puzzle = Arc::clone(&self.puzzle);
        let updater = async move {
            let input = aoc2023::get_input(day, None).await.map_err(PuzzleError)?;
            let start = std::time::Instant::now();
            let mut solution = puzzle.lock().await;
            let answer = if second_part {
                solution.solve_b(input)
            } else {
                solution.solve_a(input)
            }
            .await
            .map_err(PuzzleError)?;
            let time = start.elapsed();
            update_callback();
            Ok((answer, time))
        };
        ImmediateValuePromise::new(updater)
    }

    fn fetch_shapes(&mut self, rect: egui::Rect) -> PuzzleShapesPromise {
        let day = self.day;
        let update_callback = self.update_callback();
        let puzzle = Arc::clone(&self.puzzle);
        let updater = async move {
            let input = aoc2023::get_input(day, None).await.map_err(PuzzleError)?;
            let mut solution = puzzle.lock().await;
            let shapes = solution
                .get_shapes(input, rect)
                .await
                .ok_or(PuzzleError("No visualization available".into()))?;
            update_callback();
            Ok(shapes)
        };
        ImmediateValuePromise::new(updater)
    }
}

fn display_result(ui: &mut Ui, result: &mut Option<PuzzleAnswerPromise>) {
    if let Some(state) = result {
        match state.poll_state() {
            lazy_async_promise::ImmediateValueState::Updating => {
                ui.spinner();
                ui.label("Calculating...");
            }
            lazy_async_promise::ImmediateValueState::Success((answer, duration)) => {
                ui.label("Answer: ");
                let label = ui.selectable_label(false, answer.get_result());
                let label = label.on_hover_text("Copy to clipboard");
                if label.clicked() {
                    ui.output_mut(|o| o.copied_text = answer.get_result());
                }
                ui.label(format!("{:.2?}", duration));
            }
            lazy_async_promise::ImmediateValueState::Error(err) => {
                ui.label(format!("Error: {:?}", err.to_string()));
            }
            lazy_async_promise::ImmediateValueState::Empty => {}
        };
    }
}

fn display_shapes(
    painter: &egui::Painter,
    to_screen: emath::RectTransform,
    result: &mut Option<PuzzleShapesPromise>,
) {
    use egui::{epaint::*, *};

    if let Some(state) = result {
        match state.poll_state() {
            lazy_async_promise::ImmediateValueState::Updating => {
                painter.text(
                    painter.clip_rect().center(),
                    Align2::CENTER_CENTER,
                    "Fetching data...",
                    FontId::proportional(20.0),
                    Color32::WHITE,
                );
            }
            lazy_async_promise::ImmediateValueState::Success(shapes) => {
                painter.extend(shapes.iter().cloned().map(|s| s.into_native(to_screen)));
                shapes.iter().for_each(|s| s.paint(painter, to_screen));
            }
            lazy_async_promise::ImmediateValueState::Error(err) => {
                painter.text(
                    painter.clip_rect().center(),
                    Align2::CENTER_CENTER,
                    err.to_string(),
                    FontId::proportional(20.0),
                    Color32::RED,
                );
            }
            lazy_async_promise::ImmediateValueState::Empty => {}
        };
    }
}

impl PuzzleViewportUi for PuzzleViewport {
    fn get_day(&self) -> u8 {
        self.day
    }
    fn update(&mut self, ctx: &egui::Context) {
        self.update_callback_ctx = Some(ctx.clone());
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Day {:02}", self.day));
            ui.horizontal(|ui| {
                if ui.button("Solve A").clicked() {
                    self.part_a = Some(self.solve_part(false));
                }
                display_result(ui, &mut self.part_a);
            });
            ui.horizontal(|ui| {
                if ui.button("Solve B").clicked() {
                    self.part_b = Some(self.solve_part(true));
                }
                display_result(ui, &mut self.part_b);
            });
            ui.separator();
            let mut remaining_space = ui.available_size();
            remaining_space.y = remaining_space.y - 18.0;
            ui.horizontal(|ui| {
                ui.set_height(18.0);
                if ui.button("Try visualize").clicked() {
                    self.visualization_offset = emath::Vec2::ZERO;
                    self.shapes = Some(self.fetch_shapes(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        remaining_space,
                    )));
                }
                ui.add(egui::Slider::new(&mut self.visualization_zoom, 25.0..=3000.0).suffix("%"));
            });
            ui.with_layout(egui::Layout::default(), |ui| {
                let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::drag());
                self.visualization_offset += response.drag_delta();
                let placement_rect = response.rect.translate(self.visualization_offset);
                let scale = self.visualization_zoom / 100.0;
                let from = egui::Rect::from_x_y_ranges(0.0..=1.0, 0.0..=1.0);
                let to = egui::Rect::from_x_y_ranges(
                    placement_rect.left()..=placement_rect.left() + scale,
                    placement_rect.top()..=placement_rect.top() + scale,
                );
                let to_screen = emath::RectTransform::from_to(from, to);
                display_shapes(
                    &painter,
                    to_screen,
                    &mut self.shapes,
                );
            });
        });
    }
}
