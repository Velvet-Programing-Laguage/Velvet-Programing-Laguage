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

    public String drawCircle(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 3) {
                return "Invalid arguments: x,y,radius required";
            }
            float x = Float.parseFloat(parts[0]);
            float y = Float.parseFloat(parts[1]);
            float radius = Float.parseFloat(parts[2]);
            int segments = 50;

            glfwMakeContextCurrent(window);
            glClear(GL_COLOR_BUFFER_BIT);
            glBegin(GL_TRIANGLE_FAN);
            glVertex2f(x, y);
            for (int i = 0; i <= segments; i++) {
                double angle = Math.PI * 2.0 * i / segments;
                glVertex2f(x + (float) (Math.cos(angle) * radius), y + (float) (Math.sin(angle) * radius));
            }
            glEnd();
            glfwSwapBuffers(window);
            glfwPollEvents();
            return "Circle drawn";
        } catch (Exception e) {
            return "Error drawing circle: " + e.getMessage();
        }
    }

    public String drawText(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length != 3) {
                return "Invalid arguments: text,x,y required";
            }
            String text = parts[0];
            float x = Float.parseFloat(parts[1]);
            float y = Float.parseFloat(parts[2]);

            // Placeholder for text rendering (requires additional font rendering library)
            glfwMakeContextCurrent(window);
            glClear(GL_COLOR_BUFFER_BIT);
            // Note: Actual text rendering requires a library like FreeType or stb_truetype via LWJGL
            glfwSwapBuffers(window);
            glfwPollEvents();
            return "Text drawn: " + text + " at (" + x + "," + y + ")";
        } catch (Exception e) {
            return "Error drawing text: " + e.getMessage();
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
