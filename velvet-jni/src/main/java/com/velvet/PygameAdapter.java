package com.velvet;

import org.lwjgl.opengl.GL;
import static org.lwjgl.opengl.GL11.*;

public class PygameAdapter {
    public String init(String args) {
        // Initialize OpenGL context (simulating Pygame-like functionality)
        GL.createCapabilities();
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        return "Pygame adapter initialized with OpenGL";
    }
}
