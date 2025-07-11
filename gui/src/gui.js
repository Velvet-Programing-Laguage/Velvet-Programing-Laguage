const { invoke } = window.__TAURI__.tauri;

export function renderComponent(type, id, x, y, options = {}) {
    const container = document.getElementById('components');
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
                document.dispatchEvent(new CustomEvent('velvet:action', { detail: { id, result } }));
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
            element.addEventListener('change', () => {
                document.dispatchEvent(new CustomEvent('velvet:change', { detail: { id, value: element.value } }));
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
            element.addEventListener('input', () => {
                document.dispatchEvent(new CustomEvent('velvet:input', { detail: { id, value: element.value } }));
            });
            break;
        case 'grid':
            element = document.createElement('div');
            element.className = 'velvet-grid';
            element.style.left = `${x}px`;
            element.style.top = `${y}px`;
            element.style.gridTemplateRows = `repeat(${options.rows}, 1fr)`;
            element.style.gridTemplateColumns = `repeat(${options.cols}, 1fr)`;
            break;
        case 'dialog':
            element = document.createElement('div');
            element.className = 'velvet-dialog';
            element.innerHTML = `
                <div class="velvet-dialog-content">
                    <span class="velvet-dialog-close">×</span>
                    <p>${options.content || 'Dialog Content'}</p>
                </div>
            `;
            element.style.left = `${x}px`;
            element.style.top = `${y}px`;
            element.querySelector('.velvet-dialog-close').addEventListener('click', () => {
                element.style.display = 'none';
            });
            break;
        case 'tab':
            element = document.createElement('div');
            element.className = 'velvet-tab';
            options.tabs.forEach((tab, index) => {
                const tabButton = document.createElement('button');
                tabButton.textContent = tab.name;
                tabButton.className = 'velvet-tab-button';
                tabButton.addEventListener('click', () => {
                    document.querySelectorAll('.velvet-tab-content').forEach(c => c.style.display = 'none');
                    document.getElementById(`tab-content-${id}-${index}`).style.display = 'block';
                });
                element.appendChild(tabButton);
                const content = document.createElement('div');
                content.id = `tab-content-${id}-${index}`;
                content.className = 'velvet-tab-content';
                content.textContent = tab.content;
                content.style.display = index === 0 ? 'block' : 'none';
                element.appendChild(content);
            });
            element.style.left = `${x}px`;
            element.style.top = `${y}px`;
            break;
    }
    element.id = id;
    container.appendChild(element);
}
