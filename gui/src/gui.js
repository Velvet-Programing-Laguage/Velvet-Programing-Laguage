const { invoke } = window.__TAURI__.tauri;

export function renderComponent(type, id, x, y, options = {}) {
    const container = document.getElementById('app');
    let element;
    switch (type) {
        case 'button':
            element = document.createElement('button');
            element.textContent = options.text || 'Click Me';
            element.style.left = `${x}px`;
            element.style.top = `${y}px`;
            element.className = 'velvet-button';
            element.addEventListener('click', async () => {
                const result = await invoke('velvet_' + options.action, { args: options.command });
                console.log(`Button ${id} action:`, result);
            });
            break;
        case 'dropdown':
            element = document.createElement('select');
            options.items.forEach(item => {
                const opt = document.createElement('option');
                opt.value = item;
                opt.textContent = item;
                element.appendChild(opt);
            });
            element.style.left = `${x}px`;
            element.style.top = `${y}px`;
            element.className = 'velvet-dropdown';
            element.addEventListener('change', async () => {
                console.log(`Dropdown ${id} selected:`, element.value);
            });
            break;
        case 'slider':
            element = document.createElement('input');
            element.type = 'range';
            element.min = options.min || '0';
            element.max = options.max || '100';
            element.style.left = `${x}px`;
            element.style.top = `${y}px`;
            element.className = 'velvet-slider';
            element.addEventListener('input', async () => {
                console.log(`Slider ${id} value:`, element.value);
            });
            break;
    }
    element.id = id;
    container.appendChild(element);
}
