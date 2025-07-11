import { renderComponent } from './gui.js';

document.addEventListener('velvet:initialized', () => {
    // Render default components
    renderComponent('button', 'btn1', 0, 0, { 
        text: 'Fetch Data', 
        action: 'js_axios', 
        command: 'axios_get,https://api.example.com' 
    });
    renderComponent('dropdown', 'theme', 0, 100, { 
        items: ['Light', 'Dark', 'System'] 
    });
    renderComponent('slider', 'volume', 0, 200, { min: 0, max: 100 });
    renderComponent('tab', 'tabs1', 0, 300, {
        tabs: [
            { name: 'Python', content: 'Python Requests Module' },
            { name: 'Ruby', content: 'Ruby HTTParty Module' }
        ]
    });
    renderComponent('modal', 'modal1', 0, 0, { 
        content: 'Welcome to Velvet!' 
    });
});

document.addEventListener('velvet:action', (e) => {
    console.log(`Action from ${e.detail.id}:`, e.detail.result);
});

document.getElementById('module-toggle').addEventListener('click', () => {
    const modal = document.getElementById('modal1');
    modal.style.display = modal.style.display === 'none' ? 'flex' : 'none';
});
