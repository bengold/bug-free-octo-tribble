use crate::css::{Color, Value};
use crate::dom::NodeType;
use crate::layout::{BoxType, LayoutBox, Rect};

pub type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
    Text(String, Rect, Color),
}

/// Build a display list from a layout tree
pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    render_text(list, layout_box);
    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color| {
        list.push(DisplayCommand::SolidColor(
            color,
            layout_box.dimensions.border_box(),
        ))
    });
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return,
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: d.border.left,
            height: border_box.height,
        },
    ));

    // Right border
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x + border_box.width - d.border.right,
            y: border_box.y,
            width: d.border.right,
            height: border_box.height,
        },
    ));

    // Top border
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: d.border.top,
        },
    ));

    // Bottom border
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y + border_box.height - d.border.bottom,
            width: border_box.width,
            height: d.border.bottom,
        },
    ));
}

fn render_text(list: &mut DisplayList, layout_box: &LayoutBox) {
    // Check if this layout box corresponds to a text node
    let style_node = match layout_box.box_type {
        BoxType::InlineNode(style) | BoxType::BlockNode(style) => style,
        BoxType::AnonymousBlock => return,
    };

    // Extract text content if this is a text node
    if let NodeType::Text(text) = &style_node.node.node_type {
        let text = text.trim();
        if text.is_empty() {
            return;
        }

        // Get text color from the styled node (which includes inheritance)
        let color = style_node.value("color")
            .and_then(|val| match val {
                Value::Color(c) => Some(c),
                _ => None,
            })
            .unwrap_or(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            });

        // Use the content box for text positioning
        let content_box = layout_box.dimensions.content;
        
        // Only render if there's actual space allocated
        if content_box.width > 0.0 && content_box.height > 0.0 {
            list.push(DisplayCommand::Text(
                text.to_string(),
                content_box,
                color,
            ));
        }
    }
}

fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => match style.value(name) {
            Some(Value::Color(color)) => Some(color),
            _ => None,
        },
        BoxType::AnonymousBlock => None,
    }
}

/// Canvas for painting
pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let white = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        Canvas {
            pixels: vec![white; width * height],
            width,
            height,
        }
    }

    pub fn paint_item(&mut self, item: &DisplayCommand) {
        match item {
            DisplayCommand::SolidColor(color, rect) => {
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..y1 {
                    for x in x0..x1 {
                        self.pixels[y * self.width + x] = *color;
                    }
                }
            }
            DisplayCommand::Text(text, rect, color) => {
                self.paint_text(text, rect, color);
            }
        }
    }

    /// Paint text as simple character blocks (placeholder for real text rendering)
    fn paint_text(&mut self, text: &str, rect: &Rect, color: &Color) {
        const CHAR_WIDTH: f32 = 8.0;
        const CHAR_HEIGHT: f32 = 12.0;
        const LINE_HEIGHT: f32 = 14.0;

        let x0 = rect.x.clamp(0.0, self.width as f32);
        let y0 = rect.y.clamp(0.0, self.height as f32);
        let max_x = (rect.x + rect.width).clamp(0.0, self.width as f32);
        let max_y = (rect.y + rect.height).clamp(0.0, self.height as f32);

        let mut x = x0;
        let mut y = y0;

        for ch in text.chars() {
            if ch == '\n' || x + CHAR_WIDTH > max_x {
                x = x0;
                y += LINE_HEIGHT;
                if ch == '\n' {
                    continue;
                }
                if y + CHAR_HEIGHT > max_y {
                    break;
                }
            }

            if ch.is_whitespace() {
                x += CHAR_WIDTH;
                continue;
            }

            // Draw a simple rectangle for each character
            let char_x0 = x as usize;
            let char_y0 = y as usize;
            let char_x1 = (x + CHAR_WIDTH).min(max_x) as usize;
            let char_y1 = (y + CHAR_HEIGHT).min(max_y) as usize;

            // Draw character as a filled rectangle with some pattern
            for py in char_y0..char_y1 {
                for px in char_x0..char_x1 {
                    // Create a simple pattern to distinguish characters
                    let is_edge = px == char_x0 || px == char_x1 - 1 || 
                                 py == char_y0 || py == char_y1 - 1;
                    if is_edge || (px + py) % 3 == 0 {
                        if py < self.height && px < self.width {
                            self.pixels[py * self.width + px] = *color;
                        }
                    }
                }
            }

            x += CHAR_WIDTH;
        }
    }

    pub fn paint(&mut self, display_list: &DisplayList) {
        for item in display_list {
            self.paint_item(item);
        }
    }
}
