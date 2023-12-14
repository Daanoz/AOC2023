use epaint::{Shape as EShape, *};

#[derive(Clone)]
pub enum DisplayData {
    NativeShape(EShape),
    TextShape(Text),
    LogLine(String),
}

impl DisplayData {
    pub fn paint(
        &self,
        painter: &egui::Painter,
        to_screen: emath::RectTransform,
    ) -> Option<EShape> {
        match self {
            Self::NativeShape(shape) => match shape {
                EShape::Rect(r) => Some(EShape::Rect(RectShape {
                    rect: to_screen.transform_rect(r.rect),
                    stroke: Stroke::new(
                        to_screen.scale().x * r.stroke.width,
                        r.stroke.color,
                    ),
                    ..*r
                })),
                EShape::Circle(c) => Some(EShape::Circle(CircleShape {
                    center: to_screen.transform_pos(c.center),
                    radius: to_screen.scale().x * (c.radius),
                    stroke: Stroke::new(
                        to_screen.scale().x * c.stroke.width,
                        c.stroke.color,
                    ),
                    ..*c
                })),
                EShape::LineSegment { points, stroke } => Some(EShape::LineSegment {
                    points: [
                        to_screen.transform_pos(points[0]),
                        to_screen.transform_pos(points[1]),
                    ],
                    stroke: Stroke::new(
                        to_screen.scale().x * stroke.width,
                        stroke.color,
                    ),
                }),
                EShape::Path(path) => Some(EShape::Path(PathShape {
                    points: path
                        .points
                        .iter()
                        .map(|p| to_screen.transform_pos(*p))
                        .collect(),
                    stroke: Stroke::new(
                        to_screen.scale().x * path.stroke.width,
                        path.stroke.color,
                    ),
                    ..*path
                })),
                _ => panic!("Unsupported shape type"),
            },
            Self::TextShape(text) => {
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
                None
            }
            Self::LogLine(_) => None,
        }
    }

    pub fn rect(&self) -> Rect {
        match self {
            Self::NativeShape(shape) => shape.visual_bounding_rect(),
            Self::TextShape(text) => {
                let size = text.size;
                let width = size * text.text.len() as f32;
                let height = size;
                Rect::from_min_max(text.pos, text.pos + vec2(width, height))
            }
            Self::LogLine(_) => Rect::ZERO,
        }
    }

    pub fn pixel(pos: Pos2, color: Color32) -> Self {
        Self::NativeShape(EShape::Rect(RectShape::filled(
            Rect::from_min_size(pos, Vec2::splat(1.0)),
            Rounding::ZERO,
            color,
        )))
    }
    pub fn text(pos: Pos2, text: String, size: f32, color: Color32) -> Self {
        Self::TextShape(Text {
            pos,
            text,
            size,
            color,
        })
    }
    pub fn log_line(text: String) -> Self {
        Self::LogLine(text)
    }
}

impl From<EShape> for DisplayData {
    fn from(shape: EShape) -> Self {
        Self::NativeShape(shape)
    }
}

#[derive(Clone)]
pub struct Text {
    pos: Pos2,
    text: String,
    size: f32,
    color: Color32,
}
