const { invoke } = window.__TAURI__.tauri;

async function initializeApp() {
    try {
        const result = await invoke('velvet_init', { args: '' });
        console.log('Velvet initialized:', result);
        await loadModules();
        document.dispatchEvent(new Event('velvet:initialized'));
    } catch (error) {
        console.error('Initialization failed:', error);
    }
}

async function loadModules() {
    const modules = [
        'python_requests', 'cpp_boost', 'csharp_json',
        'ruby_httparty', 'js_axios', 'rust_flate2', 'java_jython'
    ];
    for (const mod of modules) {
        const result = await invoke(`velvet_${mod}`, { args: 'init' });
        console.log(`Module ${mod} loaded:`, result);
    }
}

window.addEventListener('DOMContentLoaded', initializeApp);
