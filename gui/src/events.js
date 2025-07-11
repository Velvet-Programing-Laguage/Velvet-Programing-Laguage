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
    renderComponent('grid', 'grid1', 0, 300, { rows: 2, cols: 3 });
    renderComponent('dialog', 'dialog1', 0, 0, { 
        content: 'Welcome to Velvet! Try AI or Wayland features.' 
    });
    renderComponent('tab', 'tabs1', 0, 400, {
        tabs: [
            { name: 'AI', content: 'TensorFlow & PyTorch Modules' },
            { name: 'Performance', content: 'Parallel & Crypto Modules' }
        ]
    });
});

document.addEventListener('velvet:action', (e) => {
    console.log(`Action from ${e.detail.id}:`, e.detail.result);
    const dialog = document.getElementById('dialog1');
    dialog.querySelector('.velvet-dialog-content p').textContent = e.detail.result;
    dialog.style.display = 'flex';
});

document.getElementById('module-toggle').addEventListener('click', () => {
    const dialog = document.getElementById('dialog1');
    dialog.style.display = dialog.style.display === 'none' ? 'flex' : 'none';
});
