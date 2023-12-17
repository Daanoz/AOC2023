mod display_data;
mod display_request;
mod display_result;

pub use display_data::DisplayData;
pub use display_request::DisplayRequest;
pub use display_result::DisplayResult;

use egui::epaint::Pos2;

pub fn render_grid<T, CB>(grid: &Vec<Vec<T>>, f: CB) -> Vec<DisplayData>
where
    CB: Fn(&T, Pos2) -> Vec<Option<DisplayData>>,
{
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .flat_map(|(x, cell)| {
                    let pos = Pos2::new(x as f32 + 0.5, y as f32 + 0.5);
                    f(cell, pos)
                })
                .filter_map(|s| s)
                .collect::<Vec<DisplayData>>()
        })
        .map(|s| s.into())
        .collect::<Vec<DisplayData>>()
}
