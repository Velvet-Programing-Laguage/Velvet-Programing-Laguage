package com.velvet;

import javax.swing.*;
import java.awt.event.ActionListener;
import java.util.HashMap;
import java.util.Map;
import org.python.util.PythonInterpreter;
import com.fasterxml.jackson.databind.ObjectMapper;

public class TkAdapter {
    private JFrame frame;
    private ButtonGroup radioGroup;
    private Map<String, JComponent> components;
    private PythonInterpreter pythonInterpreter;
    private LibraryManager libraryManager;

    public TkAdapter(LibraryManager libraryManager) {
        this.components = new HashMap<>();
        this.pythonInterpreter = new PythonInterpreter();
        this.libraryManager = libraryManager;
    }

    public String init(String args) {
        try {
            frame = new JFrame("Velvet Tk Window");
            frame.setSize(600, 400);
            frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
            frame.setLayout(null);
            frame.setVisible(true);
            radioGroup = new ButtonGroup();
            return "Tk adapter initialized with Swing";
        } catch (Exception e) {
            return "Error initializing Tk adapter: " + e.getMessage();
        }
    }

    public String addLabel(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 3) return "Invalid arguments: text,x,y required";
            String text = parts[0];
            int x = Integer.parseInt(parts[1]);
            int y = Integer.parseInt(parts[2]);

            JLabel label = new JLabel(text);
            label.setBounds(x, y, 200, 30);
            frame.add(label);
            components.put("label_" + text, label);
            frame.repaint();
            return "Label added: " + text;
        } catch (Exception e) {
            return "Error adding label: " + e.getMessage();
        }
    }

    public String addButton(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 4) return "Invalid arguments: text,x,y,action required";
            String text = parts[0];
            int x = Integer.parseInt(parts[1]);
            int y = Integer.parseInt(parts[2]);
            String action = parts[3];

            JButton button = new JButton(text);
            button.setBounds(x, y, 100, 30);
            button.addActionListener(e -> {
                System.out.println("Button clicked: " + text);
                libraryManager.handleAction(action);
                pythonInterpreter.exec("print('Button " + text + " clicked')");
            });
            frame.add(button);
            components.put("button_" + text, button);
            frame.repaint();
            return "Button added: " + text;
        } catch (Exception e) {
            return "Error adding button: " + e.getMessage();
        }
    }

    public String addDropdown(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length < 4) return "Invalid arguments: id,x,y,options required";
            String id = parts[0];
            int x = Integer.parseInt(parts[1]);
            int y = Integer.parseInt(parts[2]);
            String[] options = parts[3].split("\\|");

            JComboBox<String> dropdown = new JComboBox<>(options);
            dropdown.setBounds(x, y, 200, 30);
            dropdown.setName(id);
            dropdown.addActionListener(e -> {
                System.out.println("Dropdown " + id + " selected: " + dropdown.getSelectedItem());
                pythonInterpreter.set("dropdown_" + id, dropdown.getSelectedItem());
                pythonInterpreter.exec("print('Dropdown " + id + " selected: ' + dropdown_" + id + ")");
            });
            frame.add(dropdown);
            components.put("dropdown_" + id, dropdown);
            frame.repaint();
            return "Dropdown added: " + id;
        } catch (Exception e) {
            return "Error adding dropdown: " + e.getMessage();
        }
    }

    public String addSlider(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 5) return "Invalid arguments: id,x,y,min,max required";
            String id = parts[0];
            int x = Integer.parseInt(parts[1]);
            int y = Integer.parseInt(parts[2]);
            int min = Integer.parseInt(parts[3]);
            int max = Integer.parseInt(parts[4]);

            JSlider slider = new JSlider(JSlider.HORIZONTAL, min, max, min);
            slider.setBounds(x, y, 200, 50);
            slider.setName(id);
            slider.addChangeListener(e -> {
                System.out.println("Slider " + id + " value: " + slider.getValue());
                pythonInterpreter.set("slider_" + id, slider.getValue());
                pythonInterpreter.exec("print('Slider " + id + " set to: ' + str(slider_" + id + "))");
            });
            frame.add(slider);
            components.put("slider_" + id, slider);
            frame.repaint();
            return "Slider added: " + id;
        } catch (Exception e) {
            return "Error adding slider: " + e.getMessage();
        }
    }

    public String addTable(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 5) return "Invalid arguments: id,x,y,rows,cols required";
            String id = parts[0];
            int x = Integer.parseInt(parts[1]);
            int y = Integer.parseInt(parts[2]);
            int rows = Integer.parseInt(parts[3]);
            int cols = Integer.parseInt(parts[4]);

            String[][] data = new String[rows][cols];
            String[] headers = new String[cols];
            for (int i = 0; i < cols; i++) headers[i] = "Col " + (i + 1);
            JTable table = new JTable(data, headers);
            JScrollPane scrollPane = new JScrollPane(table);
            scrollPane.setBounds(x, y, 300, 150);
            frame.add(scrollPane);
            components.put("table_" + id, scrollPane);
            frame.repaint();
            return "Table added: " + id;
        } catch (Exception e) {
            return "Error adding table: " + e.getMessage();
        }
    }

    public String updateProgressBar(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 2) return "Invalid arguments: id,value required";
            String id = parts[0];
            int value = Integer.parseInt(parts[1]);

            JProgressBar progressBar = (JProgressBar) components.get("progress_" + id);
            if (progressBar == null) return "Progress bar not found: " + id;
            progressBar.setValue(value);
            frame.repaint();
            return "Progress bar updated: " + id + " to " + value;
        } catch (Exception e) {
            return "Error updating progress bar: " + e.getMessage();
        }
    }

    public String setWindowSize(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 2) return "Invalid arguments: width,height required";
            int width = Integer.parseInt(parts[0]);
            int height = Integer.parseInt(parts[1]);

            frame.setSize(width, height);
            frame.revalidate();
            return "Window size set to: " + width + "x" + height;
        } catch (Exception e) {
            return "Error setting window size: " + e.getMessage();
        }
    }

    public String setWindowTitle(String args) {
        try {
            frame.setTitle(args);
            frame.revalidate();
            return "Window title set to: " + args;
        } catch (Exception e) {
            return "Error setting window title: " + e.getMessage();
        }
    }
}
