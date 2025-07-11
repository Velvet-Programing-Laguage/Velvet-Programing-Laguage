import { renderComponent } from './gui.js';

document.addEventListener('DOMContentLoaded', () => {
    // Example: Render components dynamically
    renderComponent('button', 'btn1', 50, 50, { text: 'Fetch Data', action: 'js_axios', command: 'axios_get,https://api.example.com' });
    renderComponent('dropdown', 'theme', 50, 100, { items: ['light', 'dark', 'system'] });
    renderComponent('slider', 'volume', 50, 150, { min: 0, max: 100 });
});
