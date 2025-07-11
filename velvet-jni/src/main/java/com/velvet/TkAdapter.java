package com.velvet;

import javax.swing.*;
import java.awt.event.ActionListener;
import org.python.util.PythonInterpreter;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.HashMap;
import java.util.Map;

public class TkAdapter {
    private JFrame frame;
    private ButtonGroup radioGroup;
    private Map<String, JComponent> components;
    private PythonInterpreter pythonInterpreter;

    public TkAdapter() {
        components = new HashMap<>();
        pythonInterpreter = new PythonInterpreter();
    }

    public String init(String args) {
        try {
            frame = new JFrame("Velvet Tk Window");
            frame.setSize(400, 300);
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
            if (parts.length != 3) {
                return "Invalid arguments: text,x,y required";
            }
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

    public String addMenu(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length < 1) {
                return "Invalid arguments: menu items required";
            }
            JMenuBar menuBar = new JMenuBar();
            JMenu menu = new JMenu("Menu");
            for (String item : parts) {
                JMenuItem menuItem = new JMenuItem(item);
                menuItem.addActionListener(e -> {
                    System.out.println("Menu item clicked: " + item);
                    pythonInterpreter.exec("print('Menu item " + item + " clicked from Jython')");
                });
                menu.add(menuItem);
            }
            menuBar.add(menu);
            frame.setJMenuBar(menuBar);
            frame.revalidate();
            return "Menu added";
        } catch (Exception e) {
            return "Error adding menu: " + e.getMessage();
        }
    }

    public String addCheckbox(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 3) {
                return "Invalid arguments: text,x,y required";
            }
            String text = parts[0];
            int x = Integer.parseInt(parts[1]);
            int y = Integer.parseInt(parts[2]);

            JCheckBox checkbox = new JCheckBox(text);
            checkbox.setBounds(x, y, 200, 30);
            checkbox.addActionListener(e -> {
                System.out.println("Checkbox state changed: " + text + " is " + (checkbox.isSelected() ? "checked" : "unchecked"));
                pythonInterpreter.exec("print('Checkbox " + text + " state changed to " + (checkbox.isSelected() ? "checked" : "unchecked") + "')");
            });
            frame.add(checkbox);
            components.put("checkbox_" + text, checkbox);
            frame.repaint();
            return "Checkbox added: " + text;
        } catch (Exception e) {
            return "Error adding checkbox: " + e.getMessage();
        }
    }

    public String addRadioButton(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 4) {
                return "Invalid arguments: text,x,y,group required";
            }
            String text = parts[0];
            int x = Integer.parseInt(parts[1]);
            int y = Integer.parseInt(parts[2]);
            String group = parts[3];

            JRadioButton radio = new JRadioButton(text);
            radio.setBounds(x, y, 200, 30);
            radio.setActionCommand(group);
            radio.addActionListener(e -> {
                System.out.println("Radio button selected: " + text + " in group " + group);
                pythonInterpreter.exec("print('Radio button " + text + " selected in group " + group + "')");
            });
            radioGroup.add(radio);
            frame.add(radio);
            components.put("radio_" + text, radio);
            frame.repaint();
            return "Radio button added: " + text;
        } catch (Exception e) {
            return "Error adding radio button: " + e.getMessage();
        }
    }

    public String addButton(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 3) {
                return "Invalid arguments: text,x,y required";
            }
            String text = parts[0];
            int x = Integer.parseInt(parts[1]);
            int y = Integer.parseInt(parts[2]);

            JButton button = new JButton(text);
            button.setBounds(x, y, 100, 30);
            button.addActionListener(e -> {
                System.out.println("Button clicked: " + text);
                pythonInterpreter.exec("print('Button " + text + " clicked from Jython')");
            });
            frame.add(button);
            components.put("button_" + text, button);
            frame.repaint();
            return "Button added: " + text;
        } catch (Exception e) {
            return "Error adding button: " + e.getMessage();
        }
    }

    public String addTextField(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 4) {
                return "Invalid arguments: id,placeholder,x,y required";
            }
            String id = parts[0];
            String placeholder = parts[1];
            int x = Integer.parseInt(parts[2]);
            int y = Integer.parseInt(parts[3]);

            JTextField textField = new JTextField(placeholder);
            textField.setBounds(x, y, 200, 30);
            textField.setName(id);
            textField.addActionListener(e -> {
                System.out.println("Text field input: " + id + " = " + textField.getText());
                pythonInterpreter.set("input_" + id, textField.getText());
                pythonInterpreter.exec("print('Text field " + id + " updated to: ' + input_" + id + ")");
            });
            frame.add(textField);
            components.put("text_" + id, textField);
            frame.repaint();
            return "Text field added: " + id;
        } catch (Exception e) {
            return "Error adding text field: " + e.getMessage();
        }
    }

    public String addDropdown(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length < 4) {
                return "Invalid arguments: id,x,y,options required";
            }
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

    public String addProgressBar(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 4) {
                return "Invalid arguments: id,x,y,max required";
            }
            String id = parts[0];
            int x = Integer.parseInt(parts[1]);
            int y = Integer.parseInt(parts[2]);
            int max = Integer.parseInt(parts[3]);

            JProgressBar progressBar = new JProgressBar(0, max);
            progressBar.setBounds(x, y, 200, 30);
            progressBar.setValue(0);
            progressBar.setStringPainted(true);
            frame.add(progressBar);
            components.put("progress_" + id, progressBar);
            frame.repaint();
            return "Progress bar added: " + id;
        } catch (Exception e) {
            return "Error adding progress bar: " + e.getMessage();
        }
    }

    public String updateProgressBar(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 2) {
                return "Invalid arguments: id,value required";
            }
            String id = parts[0];
            int value = Integer.parseInt(parts[1]);

            JProgressBar progressBar = (JProgressBar) components.get("progress_" + id);
            if (progressBar == null) {
                return "Progress bar not found: " + id;
            }
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
            if (parts.length != 2) {
                return "Invalid arguments: width,height required";
            }
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

    public String runJython(String args) {
        try {
            pythonInterpreter.exec(args);
            return "Jython script executed: " + args;
        } catch (Exception e) {
            return "Error executing Jython script: " + e.getMessage();
        }
    }

    public String handleLibraryAction(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length < 2) {
                return "Invalid arguments: library,action required";
            }
            String library = parts[0];
            String action = parts[1];

            switch (library) {
                case "java_jython":
                    pythonInterpreter.exec(action);
                    return "Jython action executed: " + action;
                case "csharp_json":
                    ObjectMapper mapper = new ObjectMapper();
                    if (action.startsWith("parse")) {
                        String jsonStr = parts[2];
                        mapper.readTree(jsonStr);
                        return "C# JSON parsed: " + jsonStr;
                    } else if (action.startsWith("serialize")) {
                        Map<String, String> data = new HashMap<>();
                        data.put("value", parts[2]);
                        return "C# JSON serialized: " + mapper.writeValueAsString(data);
                    }
                    break;
                default:
                    return "Unsupported library: " + library;
            }
            return "Library action processed: " + library + " - " + action;
        } catch (Exception e) {
            return "Error handling library action: " + e.getMessage();
        }
    }
}
