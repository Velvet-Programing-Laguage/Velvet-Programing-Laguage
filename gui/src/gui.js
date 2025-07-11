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

    if (props.dropdowns) {
        props.dropdowns.forEach(dropdown => {
            const select = document.createElement('select');
            select.id = `dropdown-${dropdown.id}`;
            select.className = 'window-input';
            dropdown.options.forEach(option => {
                const opt = document.createElement('option');
                opt.value = option.value;
                opt.textContent = option.label;
                select.appendChild(opt);
            });
            window.appendChild(select);
        });
    }

    if (props.progressBars) {
        props.progressBars.forEach(pb => {
            const progress = document.createElement('progress');
            progress.id = `progress-${pb.id}`;
            progress.max = pb.max || 100;
            progress.value = pb.value || 0;
            progress.className = 'window-progress';
            window.appendChild(progress);
        });
    }

    if (props.libraryActions) {
        props.libraryActions.forEach(action => {
            const btn = document.createElement('button');
            btn.id = `action-${action.id}`;
            btn.textContent = action.label || `Run ${action.type}`;
            btn.className = 'window-button';
            window.appendChild(btn);
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
            await invoke('handle_library_response', { id: act.id, data: JSON.stringify(result) });
        } else if (act.type === 'axios_get') {
            try {
                const response = await axios.get(act.url);
                await invoke('handle_library_response', { id: act.id, data: JSON.stringify(response.data) });
            } catch (error) {
                console.error('Axios error:', error);
            }
        }
    }
}
