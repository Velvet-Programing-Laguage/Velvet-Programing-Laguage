const { invoke } = window.__TAURI__.tauri;
import { renderGUI, handleEvent } from './gui.js';
import axios from 'axios';
import _ from 'lodash';

async function init() {
    // Load external JS libraries dynamically
    const libraries = [
        { name: 'axios', url: 'https://cdn.jsdelivr.net/npm/axios/dist/axios.min.js' },
        { name: 'lodash', url: 'https://cdn.jsdelivr.net/npm/lodash/lodash.min.js' },
    ];
    for (const lib of libraries) {
        if (!window[lib.name]) {
            const script = document.createElement('script');
            script.src = lib.url;
            script.async = true;
            document.head.appendChild(script);
            await new Promise(resolve => script.onload = resolve);
        }
    }

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

            if (element.props.libraryActions) {
                element.props.libraryActions.forEach(action => {
                    const actionElement = document.getElementById(`action-${action.id}`);
                    if (action.type === 'axios_get') {
                        actionElement.addEventListener('click', async () => {
                            try {
                                const response = await axios.get(action.url);
                                invoke('handle_library_response', { id: action.id, data: JSON.stringify(response.data) });
                            } catch (error) {
                                console.error('Axios error:', error);
                            }
                        });
                    } else if (action.type === 'lodash_transform') {
                        actionElement.addEventListener('click', () => {
                            const result = _.map(action.data, item => _.toUpper(item));
                            invoke('handle_library_response', { id: action.id, data: JSON.stringify(result) });
                        });
                    }
                });
            }
        }
    });
}

window.addEventListener('DOMContentLoaded', init);
