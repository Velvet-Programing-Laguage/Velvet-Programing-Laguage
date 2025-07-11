package com.velvet;

import org.lwjgl.opengl.GL;
import static org.lwjgl.opengl.GL11.*;
import static org.lwjgl.glfw.GLFW.*;

public class PygameAdapter {
    private long window;

    public String init(String args) {
        if (!glfwInit()) {
            return "Failed to initialize GLFW";
        }
        window = glfwCreateWindow(800, 600, "Velvet Pygame Window", 0, 0);
        if (window == 0) {
            glfwTerminate();
            return "Failed to create GLFW window";
        }
        glfwMakeContextCurrent(window);
        GL.createCapabilities();
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        return "Pygame adapter initialized with OpenGL";
    }

    public String drawRect(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 4) {
                return "Invalid arguments: x,y,width,height required";
            }
            float x = Float.parseFloat(parts[0]);
            float y = Float.parseFloat(parts[1]);
            float width = Float.parseFloat(parts[2]);
            float height = Float.parseFloat(parts[3]);

            glfwMakeContextCurrent(window);
            glClear(GL_COLOR_BUFFER_BIT);
            glBegin(GL_QUADS);
            glVertex2f(x, y);
            glVertex2f(x + width, y);
            glVertex2f(x + width, y + height);
            glVertex2f(x, y + height);
            glEnd();
            glfwSwapBuffers(window);
            glfwPollEvents();
            return "Rectangle drawn";
        } catch (Exception e) {
            return "Error drawing rectangle: " + e.getMessage();
        }
    }

    public String runLoop() {
        while (!glfwWindowShouldClose(window)) {
            glfwPollEvents();
            glfwSwapBuffers(window);
        }
        glfwDestroyWindow(window);
        glfwTerminate();
        return "Game loop terminated";
    }
}
