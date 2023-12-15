use eframe::egui::{self, Ui};
use lazy_async_promise::ImmediateValuePromise;
use std::{error::Error, fmt::Display, sync::Arc, time::Duration};

use aoc2023::days::Solution;
use common::Answer;
use futures::lock::Mutex;

mod visualization_data;
use visualization_data::VisualizationData;

pub trait PuzzleViewportUi {
    fn get_day(&self) -> u8;
    fn update(&mut self, ctx: &egui::Context);
}

type PuzzleAnswerPromise = ImmediateValuePromise<(Answer, Duration)>;
type PuzzleShapesPromise = ImmediateValuePromise<VisualizationData>;

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
    puzzle_visualizer: PuzzleVisualizer,
}
impl PuzzleViewport {
    pub fn new(day: u8, puzzle: Box<dyn Solution + Send>) -> Self {
        let puzzle = Arc::new(Mutex::new(puzzle));
        Self {
            day,
            puzzle: puzzle.clone(),
            update_callback_ctx: None,
            part_a: None,
            part_b: None,
            puzzle_visualizer: PuzzleVisualizer::new(day, puzzle),
        }
    }

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
            let mut solution = puzzle.lock().await;
            let start = std::time::Instant::now();
            let answer = if second_part {
                solution.solve_b(input)
            } else {
                solution.solve_a(input)
            }
            .map_err(PuzzleError)?;
            let time = start.elapsed();
            update_callback();
            Ok((answer, time))
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
            self.puzzle_visualizer.update(ctx, ui);
        });
    }
}

struct PuzzleVisualizer {
    day: u8,
    puzzle: Arc<Mutex<Box<dyn Solution + Send>>>,
    update_callback_ctx: Option<egui::Context>,

    visualization_zoom: Option<f64>,
    visualization_index: usize,
    shape_data: Option<PuzzleShapesPromise>,
}

impl PuzzleVisualizer {
    pub fn new(day: u8, puzzle: Arc<Mutex<Box<dyn Solution + Send>>>) -> Self {
        Self {
            day,
            puzzle,
            update_callback_ctx: None,

            visualization_zoom: None,
            visualization_index: 0,
            shape_data: None,
        }
    }

    fn update_callback(&self) -> impl Fn() {
        let ctx = self.update_callback_ctx.clone().unwrap();
        move || {
            ctx.request_repaint();
        }
    }

    fn update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        self.update_callback_ctx = Some(ctx.clone());
        ui.horizontal(|ui| {
            ui.set_height(18.0);
            if ui.button("Try visualize").clicked() {
                self.visualization_zoom = None;
                self.visualization_index = 0;
                self.shape_data = Some(self.fetch_shapes());
            }
            ui.add(
                egui::Slider::from_get_set(25.0..=3000.0, |val| {
                    if val.is_some() {
                        self.visualization_zoom = val;
                    }
                    self.visualization_zoom.unwrap_or(100.0)
                })
                .suffix("%"),
            );
        });
        egui::ScrollArea::both()
            .auto_shrink(false)
            .drag_to_scroll(true)
            .show(ui, |ui: &mut Ui| self.update_scroll_area(ui));
    }

    fn update_scroll_area(&mut self, ui: &mut Ui) {
        let visualization_data = if let Some(state) = self.shape_data.as_mut() {
            match state.poll_state() {
                lazy_async_promise::ImmediateValueState::Updating => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Retrieving draw data...");
                    });
                    None
                }
                lazy_async_promise::ImmediateValueState::Success(visualization_data) => {
                    Some(visualization_data)
                }
                lazy_async_promise::ImmediateValueState::Error(err) => {
                    ui.label(egui::RichText::new(err.to_string()).color(egui::Color32::RED));
                    None
                }
                lazy_async_promise::ImmediateValueState::Empty => None,
            }
        } else {
            None
        }
        .cloned();
        if let Some(visualization_data) = visualization_data {
            self.display_result(ui, &visualization_data);
        }
    }

    fn display_result(&mut self, ui: &mut Ui, visualization_data: &VisualizationData) {
        if !visualization_data.has_data {
            ui.label("No visualization available");
            return;
        }
        if let Some(result_count) = visualization_data.result.result_count {
            ui.horizontal(|ui| {
                ui.label("Select result: ");
                ui.add(egui::Slider::from_get_set(
                    0.0..=result_count as f64,
                    |val| {
                        if val.is_some() {
                            self.visualization_index = val.unwrap() as usize;
                        }
                        self.shape_data = Some(self.fetch_shapes());
                        self.visualization_index as f64
                    },
                ));
                if ui.button("-").clicked() {
                    if self.visualization_index > 0 {
                        self.visualization_index = self.visualization_index - 1;
                    } else {
                        self.visualization_index = result_count - 1;
                    }
                };
                if ui.button("+").clicked() {
                    self.visualization_index = (self.visualization_index + 1) % result_count;
                };
            });
        }
        if !visualization_data.log_lines.is_empty() {
            let log_data = visualization_data.log_lines.join("\n");
            let mut job = egui::text::LayoutJob::single_section(
                log_data,
                egui::TextFormat {
                    font_id: egui::FontId::monospace(
                        8.0 * (self.visualization_zoom.unwrap_or(100.0) as f32 / 100.0),
                    ),
                    ..Default::default()
                },
            );
            job.break_on_newline = true;
            job.wrap = egui::text::TextWrapping {
                max_width: f32::INFINITY,
                ..Default::default()
            };
            ui.add(egui::Label::new(job).wrap(false));
        }
        render_shapes(ui, self.visualization_zoom, visualization_data);
    }

    fn fetch_shapes(&mut self) -> PuzzleShapesPromise {
        let day = self.day;
        let update_callback = self.update_callback();
        let puzzle = Arc::clone(&self.puzzle);
        let request = ui_support::DisplayRequest {
            result_index: self.visualization_index,
        };
        let updater = async move {
            let input = aoc2023::get_input(day, None).await.map_err(PuzzleError)?;
            let mut solution = puzzle.lock().await;
            let shapes = solution.get_shapes(input, request).unwrap_or_default();
            let data = VisualizationData::from(shapes);
            update_callback();
            Ok(data)
        };
        ImmediateValuePromise::new(updater)
    }
}

fn render_shapes(ui: &mut Ui, zoom: Option<f64>, visualization_data: &VisualizationData) {
    use egui::{epaint::*, *};

    if visualization_data.shapes.is_empty() {
        return;
    }
    let base_scale = ui.available_size().x / visualization_data.size.x;
    let scale = if let Some(zoom) = zoom {
        base_scale * (zoom / 100.0) as f32
    } else {
        base_scale
    };
    let shapes_size = visualization_data.size * scale;
    let (response, painter) = ui.allocate_painter(shapes_size, Sense::focusable_noninteractive());
    let placement_rect = response.rect;
    let from = egui::Rect::from_x_y_ranges(0.0..=1.0, 0.0..=1.0);
    let to = egui::Rect::from_x_y_ranges(
        placement_rect.left()..=placement_rect.left() + scale,
        placement_rect.top()..=placement_rect.top() + scale,
    );
    let to_screen = emath::RectTransform::from_to(from, to);

    let shapes: Vec<epaint::Shape> = visualization_data
        .shapes
        .iter()
        .filter_map(|s| s.paint(&painter, to_screen))
        .collect();
    painter.extend(shapes);
}
