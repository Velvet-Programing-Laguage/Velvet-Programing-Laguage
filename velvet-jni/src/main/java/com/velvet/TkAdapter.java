package com.velvet;

import javax.swing.JFrame;

public class TkAdapter {
    public String init(String args) {
        // Initialize Swing window (simulating Tkinter)
        JFrame frame = new JFrame("Velvet Tk Window");
        frame.setSize(400, 300);
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setVisible(true);
        return "Tk adapter initialized with Swing";
    }
}
