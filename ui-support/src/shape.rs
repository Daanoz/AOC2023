use epaint::{Shape as EShape, *};

#[derive(Clone)]
pub enum Shape {
    Native(EShape),
    Text(Text),
}

impl Shape {
    pub fn into_native(self, to_screen: emath::RectTransform) -> EShape {
        match self {
            Self::Native(shape) => match shape {
                EShape::Rect(r) => EShape::Rect(RectShape {
                    rect: to_screen.transform_rect(r.rect),
                    ..r
                }),
                _ => panic!("Unsupported shape type"),
            },
            Self::Text(_) => EShape::Noop,
        }
    }

    pub fn paint(&self, painter: &egui::Painter, to_screen: emath::RectTransform) -> () {
        match self {
            Self::Native(_) => (),
            Self::Text(text) => {
                let size = to_screen.scale().y * text.size;
                for (index, char) in text.text.chars().enumerate() {
                    let mut pos = to_screen.transform_pos(text.pos);
                    pos.x += ((index as f32) + 0.5) * size;
                    pos.y += size / 2.0;
                    painter.text(
                        pos,
                        egui::Align2::CENTER_CENTER,
                        char.to_string(),
                        FontId::monospace(size),
                        text.color,
                    );
                }
            }
        }
    }

    pub fn rect(&self) -> Rect {
        match self {
            Self::Native(shape) => shape.visual_bounding_rect(),
            Self::Text(text) => {
                let size = text.size;
                let width = size * text.text.len() as f32;
                let height = size;
                Rect::from_min_max(text.pos, text.pos + vec2(width, height))
            }
        }
    }

    pub fn pixel(pos: Pos2, color: Color32) -> Self {
        Self::Native(EShape::Rect(RectShape::filled(
            Rect::from_min_size(pos, Vec2::splat(1.0)),
            Rounding::ZERO,
            color,
        )))
    }
    pub fn text(pos: Pos2, text: String, size: f32, color: Color32) -> Self {
        Self::Text(Text {
            pos,
            text,
            size,
            color,
        })
    }
}

impl From<EShape> for Shape {
    fn from(shape: EShape) -> Self {
        Self::Native(shape)
    }
}

#[derive(Clone)]
pub struct Text {
    pos: Pos2,
    text: String,
    size: f32,
    color: Color32,
}
