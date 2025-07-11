package com.velvet;

import javax.swing.*;
import java.awt.event.ActionListener;

public class TkAdapter {
    private JFrame frame;

    public String init(String args) {
        frame = new JFrame("Velvet Tk Window");
        frame.setSize(400, 300);
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setLayout(null);
        frame.setVisible(true);
        return "Tk adapter initialized with Swing";
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
}
