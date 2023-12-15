use crate::DisplayData;

#[derive(Default, Clone)]
pub struct DisplayResult {
    pub result_count: Option<usize>,
    pub result_index: usize,
    pub shapes: Vec<DisplayData>,
}

impl From<Vec<DisplayData>> for DisplayResult {
    fn from(shapes: Vec<DisplayData>) -> Self {
        Self {
            shapes,
            ..Default::default()
        }
    }
}
