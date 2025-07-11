package com.velvet;

import org.python.util.PythonInterpreter;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.HashMap;
import java.util.Map;

public class LibraryManager {
    private PythonInterpreter pythonInterpreter;
    private ObjectMapper jsonMapper;
    private Map<String, LibraryAdapter> adapters;

    public LibraryManager() {
        pythonInterpreter = new PythonInterpreter();
        jsonMapper = new ObjectMapper();
        adapters = new HashMap<>();
        adapters.put("python_requests", new PythonRequestsAdapter());
        adapters.put("cpp_boost", new CppBoostAdapter());
        adapters.put("csharp_json", new CsharpJsonAdapter());
        adapters.put("ruby_httparty", new RubyHttpartyAdapter());
        adapters.put("js_axios", new JsAxiosAdapter());
        adapters.put("rust_flate2", new RustFlate2Adapter());
        adapters.put("java_jython", new JavaJythonAdapter(pythonInterpreter));
    }

    public String handleAction(String action) {
        try {
            String[] parts = action.split(",", 2);
            String library = parts[0];
            String command = parts.length > 1 ? parts[1] : "";
            LibraryAdapter adapter = adapters.get(library);
            if (adapter == null) return "Unknown library: " + library;
            return adapter.execute(command);
        } catch (Exception e) {
            return "Error handling action: " + e.getMessage();
        }
    }
}

interface LibraryAdapter {
    String execute(String command);
}
