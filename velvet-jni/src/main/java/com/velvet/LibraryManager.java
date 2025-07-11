package com.velvet;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.HashMap;
import java.util.Map;

public class LibraryManager {
    private final VelvetJNI jni;
    private final ObjectMapper mapper;
    private final Map<String, ModuleAdapter> adapters;

    public LibraryManager() {
        this.jni = new VelvetJNI();
        this.mapper = new ObjectMapper();
        this.adapters = new HashMap<>();
        registerAdapters();
    }

    private void registerAdapters() {
        adapters.put("python_requests", new PythonRequestsAdapter());
        adapters.put("cpp_boost", new CppBoostAdapter());
        adapters.put("csharp_json", new CsharpJsonAdapter());
        adapters.put("ruby_httparty", new RubyHttpartyAdapter());
        adapters.put("js_axios", new JsAxiosAdapter());
        adapters.put("rust_flate2", new RustFlate2Adapter());
        adapters.put("java_jython", new JavaJythonAdapter());
        adapters.put("tauri_gui", new TauriGuiAdapter());
        adapters.put("wayland_gui", new WaylandGuiAdapter());
        adapters.put("ai_tensorflow", new AiTensorflowAdapter());
        adapters.put("ai_pytorch", new AiPytorchAdapter());
        adapters.put("perf_parallel", new PerfParallelAdapter());
        adapters.put("perf_crypto", new PerfCryptoAdapter());
        adapters.put("db_sqlite", new DbSqliteAdapter());
        adapters.put("net_websocket", new NetWebsocketAdapter());
        adapters.put("gpu_cuda", new GpuCudaAdapter());
    }

    public String execute(String module, String args) throws Exception {
        ModuleAdapter adapter = adapters.get(module);
        if (adapter == null) {
            throw new Exception("Module not found: " + module);
        }
        return adapter.execute(args);
    }

    public String executeAsync(String module, String args) throws Exception {
        return jni.asyncExec(module, args);
    }
}
