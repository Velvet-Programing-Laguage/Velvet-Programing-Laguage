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
            window.className = 'bg-white p-4 rounded shadow flex flex-col items-center';
            window.innerHTML = `
                <h1 class="text-xl font-bold mb-2">${item.props.title}</h1>
                ${item.props.buttons ? item.props.buttons.map(btn => `
                    <button onclick="handleAction('${btn.action[0].value}')">
                        ${btn.text}
                    </button>
                `).join('') : ''}
                ${item.props.inputs ? item.props.inputs.map(input => `
                    <input id="${input.id}" placeholder="${input.placeholder}"
                           oninput="updateInput('${input.id}', this.value)" />
                `).join('') : ''}
            `;
            app.appendChild(window);
        }
    });
}

async function handleAction(action) {
    await invoke('execute_action', { action });
}

async function updateInput(id, value) {
    await invoke('update_input', { id, value });
}

window.addEventListener('DOMContentLoaded', initApp);
