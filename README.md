# Browser Engine

A minimal web browser rendering engine built from scratch in Rust. This project implements the core components of a browser engine without using existing engines like WebKit, Chromium, or Gecko.

## Architecture

The browser engine follows a pipeline similar to real browsers:

```
HTML → DOM → Style Tree → Layout Tree → Display List → Pixels
CSS ↗
```

### Components

1. **DOM** (`src/dom.rs`)
   - Tree structure representing HTML elements
   - Element nodes with attributes
   - Text nodes

2. **HTML Parser** (`src/html.rs`)
   - Parses HTML into a DOM tree
   - Simple recursive descent parser
   - Supports elements, attributes, and text

3. **CSS Parser** (`src/css.rs`)
   - Parses CSS stylesheets
   - Supports selectors (tag, class, id)
   - Handles declarations (property: value)
   - Color values and length units

4. **Style Tree** (`src/style.rs`)
   - Combines DOM and CSS
   - Selector matching
   - Specificity calculation
   - Property inheritance

5. **Layout Engine** (`src/layout.rs`)
   - Box model implementation
   - Width/height calculation
   - Margin, padding, border
   - Block layout (no inline/flexbox yet)

6. **Painting** (`src/painting.rs`)
   - Builds display list
   - Renders backgrounds
   - Renders borders
   - Outputs to pixel buffer

## Features

**Supported:**
- HTML parsing (elements, attributes, text)
- CSS parsing (selectors, declarations)
- CSS selectors (tag, class, id)
- Box model (margin, padding, border, content)
- Block layout
- Background colors
- Border colors and widths
- PPM image output

**Not Yet Supported:**
- Inline layout
- Flexbox/Grid
- JavaScript
- Text rendering
- Images
- Events
- Networking
- Most CSS properties
- CSS cascade/inheritance (partial)

## Building and Running

```bash
# Build the project
cargo build --release

# Run the demo
cargo run

# View the output
# The program generates output.ppm which you can view with:
# - GIMP
# - ImageMagick: convert output.ppm output.png
# - Any PPM viewer
```

## Example

The engine parses example files in `examples/`:

**test.html:**
```html
<html>
<body>
  <div class="box">Hello, Browser!</div>
</body>
</html>
```

**test.css:**
```css
.box {
  width: 100px;
  height: 100px;
  background: #ff0000;
  border-width: 5px;
  border-color: #000000;
}
```

This renders a red box with a black border to `output.ppm`.

## How It Works

1. **Parse HTML** - Convert HTML text into a DOM tree
2. **Parse CSS** - Convert CSS text into style rules
3. **Style** - Match CSS rules to DOM elements
4. **Layout** - Calculate position and size of each box
5. **Paint** - Generate drawing commands
6. **Render** - Draw pixels to a canvas

## Project Structure

```
browser-engine/
├── src/
│   ├── main.rs       # Entry point
│   ├── dom.rs        # DOM tree
│   ├── html.rs       # HTML parser
│   ├── css.rs        # CSS parser
│   ├── style.rs      # Style tree
│   ├── layout.rs     # Layout engine
│   └── painting.rs   # Rendering
├── examples/
│   ├── test.html     # Example HTML
│   └── test.css      # Example CSS
└── Cargo.toml        # Dependencies
```

## Limitations

This is a toy browser engine for educational purposes. It:
- Only supports a tiny subset of HTML/CSS
- Has no error handling
- Panics on invalid input
- Doesn't handle edge cases
- Is extremely basic compared to real browsers

## Inspiration

Based on the excellent tutorial series "Let's build a browser engine!" by Matt Brubeck.

## License

See LICENSE file for details.