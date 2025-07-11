package com.velvet;

import com.fasterxml.jackson.databind.ObjectMapper;

public class CsharpJsonAdapter implements LibraryAdapter {
    private ObjectMapper jsonMapper = new ObjectMapper();

    @Override
    public String execute(String command) {
        try {
            String[] parts = command.split(",", 2);
            String method = parts[0];
            String data = parts[1];
            if (method.equals("parse")) {
                jsonMapper.readTree(data);
                return "C# JSON parsed: " + data;
            } else if (method.equals("serialize")) {
                return jsonMapper.writeValueAsString(new HashMap<String, String>() {{ put("value", data); }});
            }
            return "Unknown C# JSON method: " + method;
        } catch (Exception e) {
            return "Error in csharp_json: " + e.getMessage();
        }
    }
}
