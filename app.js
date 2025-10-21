let apps = [];
let currentIndex = 0;

// Load apps data
async function loadApps() {
    try {
        const response = await fetch('apps.json');
        apps = await response.json();

        // Sort by date (newest first)
        apps.sort((a, b) => new Date(b.date) - new Date(a.date));

        // Show the latest app
        currentIndex = 0;
        displayApp();
    } catch (error) {
        console.error('Error loading apps:', error);
        document.getElementById('app-title').textContent = 'No apps found';
        document.getElementById('app-description').textContent = 'Add apps to apps.json to get started';
    }
}

// Display current app
function displayApp() {
    if (apps.length === 0) return;

    const app = apps[currentIndex];

    // Update app info
    document.getElementById('app-title').textContent = app.title;
    document.getElementById('app-description').textContent = app.description;
    document.getElementById('app-date').textContent = new Date(app.date).toLocaleDateString();

    // Update counter
    document.getElementById('app-counter').textContent =
        `App ${currentIndex + 1} of ${apps.length}`;

    // Load app in iframe
    document.getElementById('app-frame').src = app.path;

    // Update button states
    updateButtons();
}

// Update navigation button states
function updateButtons() {
    const prevBtn = document.getElementById('prev-btn');
    const nextBtn = document.getElementById('next-btn');

    // Disable previous button if at newest
    prevBtn.disabled = currentIndex === 0;

    // Disable next button if at oldest
    nextBtn.disabled = currentIndex === apps.length - 1;
}

// Navigate to previous app (newer)
function previousApp() {
    if (currentIndex > 0) {
        currentIndex--;
        displayApp();
    }
}

// Navigate to next app (older)
function nextApp() {
    if (currentIndex < apps.length - 1) {
        currentIndex++;
        displayApp();
    }
}

// Keyboard navigation
document.addEventListener('keydown', (e) => {
    if (e.key === 'ArrowLeft') {
        previousApp();
    } else if (e.key === 'ArrowRight') {
        nextApp();
    }
});

// Event listeners
document.getElementById('prev-btn').addEventListener('click', previousApp);
document.getElementById('next-btn').addEventListener('click', nextApp);

// Initialize
loadApps();
