const { invoke } = window.__TAURI__.tauri;
import { renderGUI, handleEvent } from './gui.js';
import axios from 'axios';
import _ from 'lodash';

async function init() {
    // Load external JS libraries dynamically
    await loadJSLibrary('axios', 'https://cdn.jsdelivr.net/npm/axios/dist/axios.min.js');
    await loadJSLibrary('lodash', 'https://cdn.jsdelivr.net/npm/lodash/lodash.min.js');

    const guiData = await invoke('execute_velvet', { file: 'main.vel' });
    const container = document.getElementById('app');
    container.innerHTML = '';

    guiData.forEach(element => {
        if (element.type === 'window') {
            const window = renderGUI(element.props);
            container.appendChild(window);

            if (element.props.buttons) {
                element.props.buttons.forEach(button => {
                    const btnElement = document.getElementById(`button-${button.text}`);
                    btnElement.addEventListener('click', () => handleEvent(button.action));
                });
            }

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

async function loadJSLibrary(name, url) {
    if (window[name]) return;
    const script = document.createElement('script');
    script.src = url;
    script.async = true;
    document.head.appendChild(script);
    return new Promise(resolve => script.onload = resolve);
}

window.addEventListener('DOMContentLoaded', init);
