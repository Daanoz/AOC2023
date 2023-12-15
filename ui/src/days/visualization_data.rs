use emath::{Pos2, Rect, Vec2};
use ui_support::{DisplayData, DisplayResult};

#[derive(Clone)]
pub struct VisualizationData {
    pub has_data: bool,
    pub size: Vec2,
    pub shapes: Vec<DisplayData>,
    pub log_lines: Vec<String>,
    pub result: DisplayResult,
}

impl From<DisplayResult> for VisualizationData {
    fn from(result: DisplayResult) -> Self {
        if result.shapes.is_empty() {
            return Self {
                has_data: false,
                shapes: vec![],
                size: Vec2::ZERO,
                log_lines: Vec::new(),
                result: DisplayResult::default(),
            };
        }

        let mut size = Rect::from_two_pos(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
        let mut log_lines: Vec<String> = Vec::new();
        let shapes = result
            .shapes
            .into_iter()
            .filter(|s| {
                // extract log lines
                if let DisplayData::LogLine(data) = s {
                    log_lines.push(data.clone());
                    false
                } else {
                    true
                }
            })
            .inspect(|s| size = size.union(s.rect()))
            .collect::<Vec<DisplayData>>();
        Self {
            has_data: true,
            shapes,
            size: size.size(),
            log_lines,
            result: DisplayResult {
                shapes: vec![],
                ..result
            },
        }
    }
}
