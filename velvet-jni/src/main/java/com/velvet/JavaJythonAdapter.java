package com.velvet;

import org.python.util.PythonInterpreter;

public class JavaJythonAdapter implements LibraryAdapter {
    private PythonInterpreter pythonInterpreter;

    public JavaJythonAdapter(PythonInterpreter interpreter) {
        this.pythonInterpreter = interpreter;
    }

    @Override
    public String execute(String command) {
        try {
            pythonInterpreter.exec(command);
            return "Jython script executed: " + command;
        } catch (Exception e) {
            return "Error in java_jython: " + e.getMessage();
        }
    }
}
