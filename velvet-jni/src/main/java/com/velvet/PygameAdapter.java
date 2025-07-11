package com.velvet;

import org.lwjgl.opengl.GL;
import static org.lwjgl.opengl.GL11.*;
import static org.lwjgl.glfw.GLFW.*;
import org.apache.http.client.methods.HttpGet;
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.http.impl.client.HttpClients;
import org.apache.http.util.EntityUtils;

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

            glfwMakeContextCurrent(window);
            glClear(GL_COLOR_BUFFER_BIT);
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

    public String drawFromLibrary(String args) {
        try {
            String[] parts = args.split(",");
            if (parts.length < 2) {
                return "Invalid arguments: library,action required";
            }
            String library = parts[0];
            String action = parts[1];
            if (library.equals("js_axios") && action.startsWith("get")) {
                String url = parts[2];
                try (CloseableHttpClient client = HttpClients.createDefault()) {
                    HttpGet request = new HttpGet(url);
                    String response = EntityUtils.toString(client.execute(request).getEntity());
                    return "Axios GET result: " + response;
                }
            }
            return "Drawing with " + library + ": " + action;
        } catch (Exception e) {
            return "Error using library for drawing: " + e.getMessage();
        }
    }
}
