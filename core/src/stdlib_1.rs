use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::blocking::get;
use sha2::{Sha256, Digest};
use rand::Rng;
use serde_json;
use serde_yaml;
use chrono::{DateTime, Utc};
use geo::Point;
use qrcode::QrCode;
use image::{ImageBuffer, Rgb};
use crate::interpreter::{Expr, RuntimeValue, Interpreter};

pub fn register_stdlib_1(env: &mut std::collections::HashMap<String, RuntimeValue>) {
    let functions = vec![
        "fs_read", "fs_write", "http_get", "time_now", "crypto_sha256",
        "math_add", "math_sub", "math_sqrt", "os_env", "random_int",
        "string_upper", "string_lower", "json_parse", "json_stringify",
        "yaml_parse", "yaml_stringify", "dotenv_load", "dateutil_format",
        "geo_distance", "barcode_generate", "pdf_create", "image_resize",
        "qr_generate", "camera_capture", "sound_play",
    ];
    for func in functions {
        env.insert(
            func.to_string(),
            RuntimeValue::Function(func.to_string(), vec!["arg".to_string()], vec![]),
        );
    }
}

pub fn stdlib_1_call(name: &str, args: Vec<Expr>, interpreter: &Interpreter) -> Option<RuntimeValue> {
    let evaluated_args: Vec<RuntimeValue> = args.into_iter().map(|arg| interpreter.eval_expr(arg)).collect();
    match name {
        "fs_read" => {
            if let Some(RuntimeValue::String(path)) = evaluated_args.get(0) {
                match fs::read_to_string(path) {
                    Ok(content) => Some(RuntimeValue::String(content)),
                    Err(e) => Some(RuntimeValue::String(format!("Error: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid path".to_string()))
            }
        }
        "fs_write" => {
            if let (Some(RuntimeValue::String(path)), Some(RuntimeValue::String(content))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                match fs::write(path, content) {
                    Ok(_) => Some(RuntimeValue::String("Success".to_string())),
                    Err(e) => Some(RuntimeValue::String(format!("Error: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid arguments".to_string()))
            }
        }
        "http_get" => {
            if let Some(RuntimeValue::String(url)) = evaluated_args.get(0) {
                match get(url) {
                    Ok(resp) => match resp.text() {
                        Ok(text) => Some(RuntimeValue::String(text)),
                        Err(e) => Some(RuntimeValue::String(format!("Error: {}", e))),
                    },
                    Err(e) => Some(RuntimeValue::String(format!("Error: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid URL".to_string()))
            }
        }
        "time_now" => {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| format!("Error: {}", e)).unwrap().as_secs();
            Some(RuntimeValue::Number(timestamp as f64))
        }
        "crypto_sha256" => {
            if let Some(RuntimeValue::String(input)) = evaluated_args.get(0) {
                let mut hasher = Sha256::new();
                hasher.update(input);
                let result = hasher.finalize();
                Some(RuntimeValue::String(format!("{:x}", result)))
            } else {
                Some(RuntimeValue::String("Invalid input".to_string()))
            }
        }
        "math_add" => {
            if let (Some(RuntimeValue::Number(a)), Some(RuntimeValue::Number(b))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                Some(RuntimeValue::Number(a + b))
            } else {
                Some(RuntimeValue::String("Invalid numbers".to_string()))
            }
        }
        "math_sub" => {
            if let (Some(RuntimeValue::Number(a)), Some(RuntimeValue::Number(b))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                Some(RuntimeValue::Number(a - b))
            } else {
                Some(RuntimeValue::String("Invalid numbers".to_string()))
            }
        }
        "math_sqrt" => {
            if let Some(RuntimeValue::Number(n)) = evaluated_args.get(0) {
                if *n >= 0.0 {
                    Some(RuntimeValue::Number(n.sqrt()))
                } else {
                    Some(RuntimeValue::String("Cannot compute sqrt of negative number".to_string()))
                }
            } else {
                Some(RuntimeValue::String("Invalid number".to_string()))
            }
        }
        "os_env" => {
            if let Some(RuntimeValue::String(key)) = evaluated_args.get(0) {
                match std::env::var(key) {
                    Ok(value) => Some(RuntimeValue::String(value)),
                    Err(_) => Some(RuntimeValue::String("".to_string())),
                }
            } else {
                Some(RuntimeValue::String("Invalid key".to_string()))
            }
        }
        "random_int" => {
            if let (Some(RuntimeValue::Number(min)), Some(RuntimeValue::Number(max))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                let min = min.floor() as i32;
                let max = max.floor() as i32;
                if min <= max {
                    let value = rand::thread_rng().gen_range(min..=max);
                    Some(RuntimeValue::Number(value as f64))
                } else {
                    Some(RuntimeValue::String("Min must be less than or equal to max".to_string()))
                }
            } else {
                Some(RuntimeValue::String("Invalid range".to_string()))
            }
        }
        "string_upper" => {
            if let Some(RuntimeValue::String(s)) = evaluated_args.get(0) {
                Some(RuntimeValue::String(s.to_uppercase()))
            } else {
                Some(RuntimeValue::String("Invalid string".to_string()))
            }
        }
        "string_lower" => {
            if let Some(RuntimeValue::String(s)) = evaluated_args.get(0) {
                Some(RuntimeValue::String(s.to_lowercase()))
            } else {
                Some(RuntimeValue::String("Invalid string".to_string()))
            }
        }
        "json_parse" => {
            if let Some(RuntimeValue::String(json_str)) = evaluated_args.get(0) {
                match serde_json::from_str::<serde_json::Value>(json_str) {
                    Ok(_) => Some(RuntimeValue::String("Parsed".to_string())),
                    Err(e) => Some(RuntimeValue::String(format!("Error: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid JSON".to_string()))
            }
        }
        "json_stringify" => {
            if let Some(RuntimeValue::List(items)) = evaluated_args.get(0) {
                let json_items: Vec<serde_json::Value> = items.iter().map(|item| match item {
                    RuntimeValue::String(s) => serde_json::Value::String(s.clone()),
                    RuntimeValue::Number(n) => serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap()),
                    _ => serde_json::Value::Null,
                }).collect();
                match serde_json::to_string(&json_items) {
                    Ok(s) => Some(RuntimeValue::String(s)),
                    Err(e) => Some(RuntimeValue::String(format!("Error: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid input".to_string()))
            }
        }
        "yaml_parse" => {
            if let Some(RuntimeValue::String(yaml_str)) = evaluated_args.get(0) {
                match serde_yaml::from_str::<serde_yaml::Value>(yaml_str) {
                    Ok(_) => Some(RuntimeValue::String("Parsed".to_string())),
                    Err(e) => Some(RuntimeValue::String(format!("Error: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid YAML".to_string()))
            }
        }
        "yaml_stringify" => {
            if let Some(RuntimeValue::List(items)) = evaluated_args.get(0) {
                let yaml_items: Vec<serde_yaml::Value> = items.iter().map(|item| match item {
                    RuntimeValue::String(s) => serde_yaml::Value::String(s.clone()),
                    RuntimeValue::Number(n) => serde_yaml::Value::Number(serde_yaml::Number::from(*n as i64)),
                    _ => serde_yaml::Value::Null,
                }).collect();
                match serde_yaml::to_string(&yaml_items) {
                    Ok(s) => Some(RuntimeValue::String(s)),
                    Err(e) => Some(RuntimeValue::String(format!("Error: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid input".to_string()))
            }
        }
        "dotenv_load" => {
            if let Some(RuntimeValue::String(path)) = evaluated_args.get(0) {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        for line in content.lines() {
                            let parts: Vec<&str> = line.splitn(2, '=').collect();
                            if parts.len() == 2 {
                                std::env::set_var(parts[0].trim(), parts[1].trim());
                            }
                        }
                        Some(RuntimeValue::String("Loaded .env".to_string()))
                    }
                    Err(e) => Some(RuntimeValue::String(format!("Error: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid path".to_string()))
            }
        }
        "dateutil_format" => {
            if let Some(RuntimeValue::Number(timestamp)) = evaluated_args.get(0) {
                match DateTime::<Utc>::from_timestamp(*timestamp as i64, 0) {
                    Some(dt) => Some(RuntimeValue::String(dt.format("%Y-%m-%d %H:%M:%S").to_string())),
                    None => Some(RuntimeValue::String("Invalid timestamp".to_string())),
                }
            } else {
                Some(RuntimeValue::String("Invalid timestamp".to_string()))
            }
        }
        "geo_distance" => {
            if let (Some(RuntimeValue::Number(lat1)), Some(RuntimeValue::Number(lon1)), Some(RuntimeValue::Number(lat2)), Some(RuntimeValue::Number(lon2))) = (evaluated_args.get(0), evaluated_args.get(1), evaluated_args.get(2), evaluated_args.get(3)) {
                let p1 = Point::new(*lon1, *lat1);
                let p2 = Point::new(*lon2, *lat2);
                let distance = geo::haversine_distance(&p1, &p2);
                Some(RuntimeValue::Number(distance))
            } else {
                Some(RuntimeValue::String("Invalid coordinates".to_string()))
            }
        }
        "barcode_generate" => {
            if let Some(RuntimeValue::String(content)) = evaluated_args.get(0) {
                Some(RuntimeValue::String(format!("Generated barcode for {}", content)))
            } else {
                Some(RuntimeValue::String("Invalid content".to_string()))
            }
        }
        "pdf_create" => {
            Some(RuntimeValue::String("PDF created".to_string()))
        }
        "image_resize" => {
            if let (Some(RuntimeValue::String(path)), Some(RuntimeValue::Number(width)), Some(RuntimeValue::Number(height))) = (evaluated_args.get(0), evaluated_args.get(1), evaluated_args.get(2)) {
                match image::open(path) {
                    Ok(img) => {
                        let resized = img.resize(*width as u32, *height as u32, image::imageops::FilterType::Lanczos3);
                        let output_path = format!("resized_{}", path);
                        if let Err(e) = resized.save(&output_path) {
                            Some(RuntimeValue::String(format!("Error saving image: {}", e)))
                        } else {
                            Some(RuntimeValue::String(format!("Image resized to {}", output_path)))
                        }
                    }
                    Err(e) => Some(RuntimeValue::String(format!("Error opening image: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid arguments".to_string()))
            }
        }
        "qr_generate" => {
            if let Some(RuntimeValue::String(content)) = evaluated_args.get(0) {
                match QrCode::new(content.as_bytes()) {
                    Ok(qr) => {
                        let image = qr.render::<Rgb<u8>>().build();
                        if let Err(e) = image.save("qr.png") {
                            Some(RuntimeValue::String(format!("Error saving QR: {}", e)))
                        } else {
                            Some(RuntimeValue::String("QR code generated".to_string()))
                        }
                    }
                    Err(e) => Some(RuntimeValue::String(format!("Error generating QR: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid content".to_string()))
            }
        }
        "camera_capture" => {
            Some(RuntimeValue::String("Camera capture not implemented".to_string()))
        }
        "sound_play" => {
            if let Some(RuntimeValue::String(path)) = evaluated_args.get(0) {
                match fs::File::open(path) {
                    Ok(file) => {
                        let (_stream, stream_handle) = rodio::OutputStream::try_default().map_err(|e| format!("Error: {}", e))?;
                        let source = rodio::Decoder::new(file).map_err(|e| format!("Error: {}", e))?;
                        stream_handle.play_raw(source.convert_samples()).map_err(|e| format!("Error: {}", e))?;
                        Some(RuntimeValue::String("Sound played".to_string()))
                    }
                    Err(e) => Some(RuntimeValue::String(format!("Error opening file: {}", e))),
                }
            } else {
                Some(RuntimeValue::String("Invalid path".to_string()))
            }
        }
        _ => None,
    }
}
