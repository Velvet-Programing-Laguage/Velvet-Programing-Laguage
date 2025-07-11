package com.velvet;

import javax.swing.*;
import java.awt.event.ActionListener;
import org.python.util.PythonInterpreter;
import com.fasterxml.jackson.databind.ObjectMapper;

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
            return "Error
