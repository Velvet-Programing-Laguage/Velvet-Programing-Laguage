const { invoke } = window.__TAURI__.tauri;
import { renderGUI, handleEvent } from './gui.js';

async function init() {
    const guiData = await invoke('execute_velvet', { file: 'main.vel' });
    const container = document.getElementById('app');
    container.innerHTML = '';

    guiData.forEach(element => {
        if (element.type === 'window') {
            const window = renderGUI(element.props);
            container.appendChild(window);

            // Attach event listeners for buttons
            if (element.props.buttons) {
                element.props.buttons.forEach(button => {
                    const btnElement = document.getElementById(`button-${button.text}`);
                    btnElement.addEventListener('click', () => handleEvent(button.action));
                });
            }

            // Attach event listeners for text inputs
            if (element.props.inputs) {
                element.props.inputs.forEach(input => {
                    const inputElement = document.getElementById(input.id);
                    inputElement.addEventListener('input', (e) => {
                        invoke('update_input', { id: input.id, value: e.target.value });
                    });
                });
            }
        }
    });
}

window.addEventListener('DOMContentLoaded', init);
