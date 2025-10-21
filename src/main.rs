mod css;
mod dom;
mod html;
mod layout;
mod painting;
mod style;

use layout::Dimensions;
use std::fs;

fn main() {
    // Read example HTML
    let html = fs::read_to_string("examples/test.html").unwrap_or_else(|_| {
        String::from("<html><body><div class='box'>Hello, Browser!</div></body></html>")
    });

    // Read example CSS
    let css = fs::read_to_string("examples/test.css").unwrap_or_else(|_| {
        String::from(
            "div { width: 100px; height: 100px; background: #ff0000; }\n\
             .box { border-width: 5px; border-color: #000000; }",
        )
    });

    println!("Parsing HTML...");
    let root_node = html::Parser::parse(html);
    println!("DOM tree:");
    println!("{:#?}", root_node);

    println!("\nParsing CSS...");
    let stylesheet = css::Parser::parse(css);
    println!("Stylesheet:");
    println!("{:#?}", stylesheet);

    println!("\nBuilding style tree...");
    let style_root = style::style_tree(&root_node, &stylesheet);
    println!("Style tree:");
    println!("{:#?}", style_root);

    println!("\nPerforming layout...");
    let mut viewport = Dimensions::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let layout_root = layout::layout_tree(&style_root, viewport);
    println!("Layout tree:");
    println!("{:#?}", layout_root);

    println!("\nBuilding display list...");
    let display_list = painting::build_display_list(&layout_root);
    println!("Display list:");
    println!("{:#?}", display_list);

    println!("\nRendering to canvas...");
    let mut canvas = painting::Canvas::new(800, 600);
    canvas.paint(&display_list);

    println!("✓ Rendering complete!");
    println!("Canvas contains {} pixels", canvas.pixels.len());

    // Save as PPM image format (simple text-based format)
    let output = format!(
        "P3\n{} {}\n255\n{}",
        canvas.width,
        canvas.height,
        canvas
            .pixels
            .iter()
            .map(|color| format!("{} {} {}", color.r, color.g, color.b))
            .collect::<Vec<_>>()
            .join(" ")
    );

    fs::write("output.ppm", output).expect("Unable to write output file");
    println!("Output saved to output.ppm");
}
