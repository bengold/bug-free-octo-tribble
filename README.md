# App Showcase

A simple website to showcase your web apps. The main page displays the latest app, with navigation to browse through previous apps one at a time.

## Features

- Clean, modern interface
- Shows one app at a time in an iframe
- Navigate between apps with Previous/Next buttons
- Keyboard navigation support (Arrow keys)
- Responsive design
- Automatically displays the newest app first

## Getting Started

1. Open `index.html` in a web browser
2. Use the Previous/Next buttons to navigate between apps
3. Or use the left/right arrow keys on your keyboard

## Adding New Apps

1. Create your app as an HTML file in the `apps/` directory
2. Add an entry to `apps.json` with the following format:

```json
{
    "title": "Your App Name",
    "description": "A brief description of your app",
    "date": "2025-10-21",
    "path": "apps/your-app.html"
}
```

The apps are automatically sorted by date, with the newest appearing first.

## Example Apps Included

- **To-Do List** - Task management app
- **Calculator** - Basic calculator with standard operations
- **Color Picker** - Pick colors and copy hex codes

## Project Structure

```
.
├── index.html          # Main showcase page
├── style.css           # Styling
├── app.js              # Navigation logic
├── apps.json           # App metadata
└── apps/               # Individual apps
    ├── todo-list.html
    ├── calculator.html
    └── color-picker.html
```

## License

See LICENSE file for details.