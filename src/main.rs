mod css;
mod dom;
mod gui;
mod html;
mod layout;
mod network;
mod painting;
mod style;

use layout::Dimensions;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let (html, css, base_url) = if args.len() > 1 {
        // URL provided as command-line argument
        let url = &args[1];
        println!("Loading URL: {}", url);
        
        match fetch_from_url(url) {
            Ok((html, css)) => (html, css, Some(url.clone())),
            Err(e) => {
                eprintln!("Error fetching URL: {}", e);
                eprintln!("Falling back to local examples...");
                load_local_files()
            }
        }
    } else {
        // No URL provided, use local files
        println!("No URL provided. Using local example files.");
        println!("Usage: {} <url>", args.get(0).unwrap_or(&String::from("browser-engine")));
        println!("Example: {} https://example.com\n", args.get(0).unwrap_or(&String::from("browser-engine")));
        load_local_files()
    };

    let _ = base_url; // Suppress unused warning for now

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

    // Save as PPM image format (simple text-based format) for backup
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

    // Display in GUI window
    println!("\nOpening GUI window...");
    match gui::BrowserWindow::new(800, 600) {
        Ok(mut window) => {
            println!("✓ GUI window created! Press ESC to close.");
            
            // Display the canvas
            if let Err(e) = window.display(&canvas) {
                eprintln!("Error displaying canvas: {:?}", e);
                return;
            }

            // Keep the window open until the user closes it
            while window.is_open() {
                // Update the window (this checks for events and keeps it responsive)
                if let Err(e) = window.display(&canvas) {
                    eprintln!("Error updating window: {:?}", e);
                    break;
                }
            }
            
            println!("Window closed.");
        }
        Err(e) => {
            eprintln!("Failed to create GUI window: {:?}", e);
            eprintln!("You can still view the output in output.ppm");
        }
    }
}

/// Load HTML and CSS from local files
fn load_local_files() -> (String, String, Option<String>) {
    let html = fs::read_to_string("examples/test.html").unwrap_or_else(|_| {
        String::from("<html><body><div class='box'>Hello, Browser!</div></body></html>")
    });

    let css = fs::read_to_string("examples/test.css").unwrap_or_else(|_| {
        String::from(
            "div { width: 100px; height: 100px; background: #ff0000; }\n\
             .box { border-width: 5px; border-color: #000000; }",
        )
    });

    (html, css, None)
}

/// Fetch HTML and CSS from a URL
fn fetch_from_url(url: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let fetcher = network::WebFetcher::new()?;
    
    // Fetch the HTML
    let html = fetcher.fetch_html(url)?;
    
    // Extract CSS links from the HTML
    let css_links = network::extract_css_links(&html);
    
    // Fetch all CSS files and combine them
    let mut combined_css = String::new();
    
    for css_link in css_links {
        // Resolve relative URLs
        let css_url = if css_link.starts_with("http://") || css_link.starts_with("https://") {
            css_link
        } else {
            match network::WebFetcher::resolve_url(url, &css_link) {
                Ok(resolved) => resolved,
                Err(e) => {
                    eprintln!("Warning: Could not resolve CSS URL {}: {}", css_link, e);
                    continue;
                }
            }
        };
        
        match fetcher.fetch_css(&css_url) {
            Ok(css) => {
                combined_css.push_str(&css);
                combined_css.push('\n');
            }
            Err(e) => {
                eprintln!("Warning: Could not fetch CSS from {}: {}", css_url, e);
            }
        }
    }
    
    // If no CSS was found, use a basic default
    if combined_css.is_empty() {
        combined_css = String::from("body { background: #ffffff; margin: 20px; }");
    }
    
    Ok((html, combined_css))
}
