package com.velvet;

public class RubyHttpartyAdapter implements LibraryAdapter {
    @Override
    public String execute(String command) {
        try {
            String[] parts = command.split(",", 2);
            String method = parts[0];
            String url = parts[1];
            // Simulate Ruby HTTParty via subprocess
            return "Ruby HTTParty " + method + " to " + url + " executed";
        } catch (Exception e) {
            return "Error in ruby_httparty: " + e.getMessage();
        }
    }
}
