const { invoke } = window.__TAURI__.tauri;

async function initApp() {
    const guiData = await invoke('get_gui_data', { file: 'main.vel' });
    renderGUI(guiData);
}

function renderGUI(data) {
    const app = document.getElementById('app');
    app.innerHTML = '';
    data.forEach(item => {
        if (item.type === 'window') {
            const window = document.createElement('div');
            window.className = 'bg-white p-6 rounded-lg shadow-lg';
            window.innerHTML = `
                <h1 class="text-2xl font-bold mb-4">${item.props.title}</h1>
                <div class="flex flex-col">
                    ${item.props.buttons.map(btn => `
                        <button class="bg-blue-500 text-white hover:bg-blue-600"
                                onclick="handleAction('${btn.action[0].value}')">
                            ${btn.text}
                        </button>
                    `).join('')}
                </div>
            `;
            app.appendChild(window);
        }
    });
}

async function handleAction(action) {
    await invoke('execute_action', { action });
}

window.addEventListener('DOMContentLoaded', initApp);
