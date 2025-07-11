package com.velvet;

import javax.swing.*;
import java.awt.event.ActionListener;
import org.python.util.PythonInterpreter;

public class TkAdapter {
    private JFrame frame;
    private ButtonGroup radioGroup;

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
                menuItem.addActionListener(e -> System.out.println("Menu item clicked: " + item));
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
            checkbox.addActionListener(e -> System.out.println("Checkbox state changed: " + text + " is " + (checkbox.isSelected() ? "checked" : "unchecked")));
            frame.add(checkbox);
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
            radio.addActionListener(e -> System.out.println("Radio button selected: " + text + " in group " + group));
            radioGroup.add(radio);
            frame.add(radio);
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
            button.addActionListener(e -> System.out.println("Button clicked: " + text));
            frame.add(button);
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
            textField.addActionListener(e -> System.out.println("Text field input: " + id + " = " + textField.getText()));
            frame.add(textField);
            frame.repaint();
            return "Text field added: " + id;
        } catch (Exception e) {
            return "Error adding text field: " + e.getMessage();
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
            PythonInterpreter interpreter = new PythonInterpreter();
            interpreter.exec(args);
            return "Jython script executed: " + args;
        } catch (Exception e) {
            return "Error executing Jython script: " + e.getMessage();
        }
    }
}
