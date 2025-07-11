export function renderGUI(props) {
    const window = document.createElement('div');
    window.className = 'window';
    window.style.width = props.size ? `${props.size[0]}px` : '800px';
    window.style.height = props.size ? `${props.size[1]}px` : '600px';

    if (props.title) {
        const title = document.createElement('h1');
        title.textContent = props.title;
        title.className = 'window-title';
        window.appendChild(title);
    }

    if (props.buttons) {
        props.buttons.forEach(button => {
            const btn = document.createElement('button');
            btn.id = `button-${button.text}`;
            btn.textContent = button.text;
            btn.className = 'window-button';
            window.appendChild(btn);
        });
    }

    if (props.inputs) {
        props.inputs.forEach(input => {
            const inp = document.createElement('input');
            inp.id = input.id;
            inp.type = 'text';
            inp.placeholder = input.placeholder;
            inp.className = 'window-input';
            window.appendChild(inp);
        });
    }

    if (props.libraryActions) {
        props.libraryActions.forEach(action => {
            if (action.type === 'axios_get') {
                const btn = document.createElement('button');
                btn.textContent = `Axios GET: ${action.url}`;
                btn.className = 'window-button';
                btn.addEventListener('click', async () => {
                    const response = await axios.get(action.url);
                    console.log('Axios response:', response.data);
                });
                window.appendChild(btn);
            }
        });
    }

    return window;
}

export async function handleEvent(action) {
    const { invoke } = window.__TAURI__.tauri;
    for (const act of action) {
        if (act.type === 'say') {
            await invoke('execute_action', { action: act.value });
        } else if (act.type === 'lodash_transform') {
            const result = _.map(act.data, item => _.toUpper(item));
            console.log('Lodash transform:', result);
        }
    }
}
