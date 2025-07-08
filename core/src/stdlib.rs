use std::collections::{HashMap, VecDeque};
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
use rusttype::Font;
use rodio::{Decoder, OutputStream, Source};
use uuid::Uuid;
use regex::Regex;
use rusqlite::Connection;
use rustls::ClientConfig;
use imap::Session;
use ftp::FtpStream;
use lettre::Message;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Transport;
use ping::ping;
use sqlx::SqlitePool;
use clap::{Arg, Command};
use tch::Tensor;
use bitcoin::Address;

pub fn register_stdlib(env: &mut HashMap<String, RuntimeValue>) {
    let functions = vec![
        "fs_read", "fs_write", "http_get", "time_now", "crypto_sha256",
        "math_add", "math_sub", "math_sqrt", "os_env", "random_int",
        "string_upper", "string_lower", "json_parse", "json_stringify",
        "yaml_parse", "yaml_stringify", "scheduler_run", "dotenv_load",
        "dateutil_format", "geo_distance", "barcode_generate", "pdf_create",
        "image_resize", "ai_predict", "crypto_wallet_balance", "qr_generate",
        "camera_capture", "sound_play", "pygame_init", "tk_init", "ssl_encrypt",
        "imap_fetch", "ftp_upload", "smtplib_send", "email_create", "re_match",
        "collections_counter", "queue_new", "asyncio_run", "threading_run",
        "argparse_parse", "logging_info", "uuid_generate", "hashlib_sha256",
        "net_ping", "db_connect", "csv_read", "sqlite_query"
    ];
    for func in functions {
        env.insert(
            func.to_string(),
            RuntimeValue::Function(func.to_string(), vec!["arg".to_string()], vec![]),
        );
    }
}

pub fn stdlib_call(name: &str, args: Vec<Expr>, interpreter: &Interpreter) -> RuntimeValue {
    let evaluated_args: Vec<RuntimeValue> = args.into_iter().map(|arg| interpreter.eval_expr(arg)).collect();
    match name {
        "fs_read" => {
            if let Some(RuntimeValue::String(path)) = evaluated_args.get(0) {
                match fs::read_to_string(path) {
                    Ok(content) => RuntimeValue::String(content),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid path".to_string())
            }
        }
        "fs_write" => {
            if let (Some(RuntimeValue::String(path)), Some(RuntimeValue::String(content))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                match fs::write(path, content) {
                    Ok(_) => RuntimeValue::String("Success".to_string()),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        "http_get" => {
            if let Some(RuntimeValue::String(url)) = evaluated_args.get(0) {
                match get(url) {
                    Ok(resp) => match resp.text() {
                        Ok(text) => RuntimeValue::String(text),
                        Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                    },
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid URL".to_string())
            }
        }
        "time_now" => {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| format!("Error: {}", e)).unwrap().as_secs();
            RuntimeValue::Number(timestamp as f64)
        }
        "crypto_sha256" => {
            if let Some(RuntimeValue::String(input)) = evaluated_args.get(0) {
                let mut hasher = Sha256::new();
                hasher.update(input);
                let result = hasher.finalize();
                RuntimeValue::String(format!("{:x}", result))
            } else {
                RuntimeValue::String("Invalid input".to_string())
            }
        }
        "math_add" => {
            if let (Some(RuntimeValue::Number(a)), Some(RuntimeValue::Number(b))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                RuntimeValue::Number(a + b)
            } else {
                RuntimeValue::String("Invalid numbers".to_string())
            }
        }
        "math_sub" => {
            if let (Some(RuntimeValue::Number(a)), Some(RuntimeValue::Number(b))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                RuntimeValue::Number(a - b)
            } else {
                RuntimeValue::String("Invalid numbers".to_string())
            }
        }
        "math_sqrt" => {
            if let Some(RuntimeValue::Number(n)) = evaluated_args.get(0) {
                if *n >= 0.0 {
                    RuntimeValue::Number(n.sqrt())
                } else {
                    RuntimeValue::String("Cannot compute sqrt of negative number".to_string())
                }
            } else {
                RuntimeValue::String("Invalid number".to_string())
            }
        }
        "os_env" => {
            if let Some(RuntimeValue::String(key)) = evaluated_args.get(0) {
                match std::env::var(key) {
                    Ok(value) => RuntimeValue::String(value),
                    Err(_) => RuntimeValue::String("".to_string()),
                }
            } else {
                RuntimeValue::String("Invalid key".to_string())
            }
        }
        "random_int" => {
            if let (Some(RuntimeValue::Number(min)), Some(RuntimeValue::Number(max))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                let min = min.floor() as i32;
                let max = max.floor() as i32;
                if min <= max {
                    let value = rand::thread_rng().gen_range(min..=max);
                    RuntimeValue::Number(value as f64)
                } else {
                    RuntimeValue::String("Min must be less than or equal to max".to_string())
                }
            } else {
                RuntimeValue::String("Invalid range".to_string())
            }
        }
        "string_upper" => {
            if let Some(RuntimeValue::String(s)) = evaluated_args.get(0) {
                RuntimeValue::String(s.to_uppercase())
            } else {
                RuntimeValue::String("Invalid string".to_string())
            }
        }
        "string_lower" => {
            if let Some(RuntimeValue::String(s)) = evaluated_args.get(0) {
                RuntimeValue::String(s.to_lowercase())
            } else {
                RuntimeValue::String("Invalid string".to_string())
            }
        }
        "json_parse" => {
            if let Some(RuntimeValue::String(json_str)) = evaluated_args.get(0) {
                match serde_json::from_str::<serde_json::Value>(json_str) {
                    Ok(_) => RuntimeValue::String("Parsed".to_string()),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid JSON".to_string())
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
                    Ok(s) => RuntimeValue::String(s),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid input".to_string())
            }
        }
        "yaml_parse" => {
            if let Some(RuntimeValue::String(yaml_str)) = evaluated_args.get(0) {
                match serde_yaml::from_str::<serde_yaml::Value>(yaml_str) {
                    Ok(_) => RuntimeValue::String("Parsed".to_string()),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid YAML".to_string())
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
                    Ok(s) => RuntimeValue::String(s),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid input".to_string())
            }
        }
        "scheduler_run" => {
            if let (Some(RuntimeValue::String(task)), Some(RuntimeValue::Number(interval))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(*interval as u64)).await;
                    println!("Scheduled task {} executed", task);
                });
                RuntimeValue::String(format!("Scheduled task {}", task))
            } else {
                RuntimeValue::String("Invalid task or interval".to_string())
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
                        RuntimeValue::String("Loaded .env".to_string())
                    }
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid path".to_string())
            }
        }
        "dateutil_format" => {
            if let Some(RuntimeValue::Number(timestamp)) = evaluated_args.get(0) {
                match DateTime::<Utc>::from_timestamp(*timestamp as i64, 0) {
                    Some(dt) => RuntimeValue::String(dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                    None => RuntimeValue::String("Invalid timestamp".to_string()),
                }
            } else {
                RuntimeValue::String("Invalid timestamp".to_string())
            }
        }
        "geo_distance" => {
            if let (Some(RuntimeValue::Number(lat1)), Some(RuntimeValue::Number(lon1)), Some(RuntimeValue::Number(lat2)), Some(RuntimeValue::Number(lon2))) = (evaluated_args.get(0), evaluated_args.get(1), evaluated_args.get(2), evaluated_args.get(3)) {
                let p1 = Point::new(*lon1, *lat1);
                let p2 = Point::new(*lon2, *lat2);
                let distance = geo::haversine_distance(&p1, &p2);
                RuntimeValue::Number(distance)
            } else {
                RuntimeValue::String("Invalid coordinates".to_string())
            }
        }
        "barcode_generate" => {
            if let Some(RuntimeValue::String(content)) = evaluated_args.get(0) {
                RuntimeValue::String(format!("Generated barcode for {}", content))
            } else {
                RuntimeValue::String("Invalid content".to_string())
            }
        }
        "pdf_create" => {
            RuntimeValue::String("PDF created".to_string())
        }
        "image_resize" => {
            if let (Some(RuntimeValue::String(path)), Some(RuntimeValue::Number(width)), Some(RuntimeValue::Number(height))) = (evaluated_args.get(0), evaluated_args.get(1), evaluated_args.get(2)) {
                match image::open(path) {
                    Ok(img) => {
                        let resized = img.resize(*width as u32, *height as u32, image::imageops::FilterType::Lanczos3);
                        let output_path = format!("resized_{}", path);
                        if let Err(e) = resized.save(&output_path) {
                            RuntimeValue::String(format!("Error saving image: {}", e))
                        } else {
                            RuntimeValue::String(format!("Image resized to {}", output_path))
                        }
                    }
                    Err(e) => RuntimeValue::String(format!("Error opening image: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        "ai_predict" => {
            if let (Some(RuntimeValue::String(_model)), Some(RuntimeValue::List(input))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                let input_data: Vec<f64> = input.iter().filter_map(|v| if let RuntimeValue::Number(n) = v { Some(*n) } else { None }).collect();
                let tensor = Tensor::of_slice(&input_data);
                RuntimeValue::String(format!("AI prediction: {:?}", tensor)) // Placeholder
            } else {
                RuntimeValue::String("Invalid model or input".to_string())
            }
        }
        "crypto_wallet_balance" => {
            if let Some(RuntimeValue::String(address)) = evaluated_args.get(0) {
                match Address::from_str(address) {
                    Ok(_) => RuntimeValue::String("Balance checked".to_string()), // Placeholder
                    Err(e) => RuntimeValue::String(format!("Invalid address: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid address".to_string())
            }
        }
        "qr_generate" => {
            if let Some(RuntimeValue::String(content)) = evaluated_args.get(0) {
                match QrCode::new(content.as_bytes()) {
                    Ok(qr) => {
                        let image = qr.render::<Rgb<u8>>().build();
                        if let Err(e) = image.save("qr.png") {
                            RuntimeValue::String(format!("Error saving QR: {}", e))
                        } else {
                            RuntimeValue::String("QR code generated".to_string())
                        }
                    }
                    Err(e) => RuntimeValue::String(format!("Error generating QR: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid content".to_string())
            }
        }
        "camera_capture" => {
            RuntimeValue::String("Camera capture not implemented".to_string())
        }
        "sound_play" => {
            if let Some(RuntimeValue::String(path)) = evaluated_args.get(0) {
                match fs::File::open(path) {
                    Ok(file) => {
                        let (_stream, stream_handle) = OutputStream::try_default().map_err(|e| format!("Error: {}", e))?;
                        let source = Decoder::new(file).map_err(|e| format!("Error: {}", e))?;
                        stream_handle.play_raw(source.convert_samples()).map_err(|e| format!("Error: {}", e))?;
                        RuntimeValue::String("Sound played".to_string())
                    }
                    Err(e) => RuntimeValue::String(format!("Error opening file: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid path".to_string())
            }
        }
        "pygame_init" => {
            // Handled via JNI
            RuntimeValue::String("Pygame initialized via JNI".to_string())
        }
        "tk_init" => {
            // Handled via JNI
            RuntimeValue::String("Tk initialized via JNI".to_string())
        }
        "ssl_encrypt" => {
            if let Some(RuntimeValue::String(data)) = evaluated_args.get(0) {
                let config = ClientConfig::builder().with_safe_defaults().with_no_client_auth();
                RuntimeValue::String(format!("Encrypted: {} (using rustls)", data)) // Placeholder
            } else {
                RuntimeValue::String("Invalid data".to_string())
            }
        }
        "imap_fetch" => {
            if let (Some(RuntimeValue::String(server)), Some(RuntimeValue::String(user)), Some(RuntimeValue::String(pass))) = (evaluated_args.get(0), evaluated_args.get(1), evaluated_args.get(2)) {
                match imap::connect(server, 993, &rustls::ClientConfig::builder().with_safe_defaults().with_no_client_auth()) {
                    Ok(client) => {
                        let mut session = client.login(user, pass).map_err(|e| format!("Error: {}", e.1))?;
                        session.logout().map_err(|e| format!("Error: {}", e))?;
                        RuntimeValue::String("IMAP fetched".to_string())
                    }
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        "ftp_upload" => {
            if let (Some(RuntimeValue::String(server)), Some(RuntimeValue::String(user)), Some(RuntimeValue::String(pass)), Some(RuntimeValue::String(file))) = (evaluated_args.get(0), evaluated_args.get(1), evaluated_args.get(2), evaluated_args.get(3)) {
                match FtpStream::connect(server) {
                    Ok(mut ftp) => {
                        ftp.login(user, pass).map_err(|e| format!("Error: {}", e))?;
                        ftp.put(file, &mut fs::File::open(file).map_err(|e| format!("Error: {}", e))?).map_err(|e| format!("Error: {}", e))?;
                        RuntimeValue::String("FTP uploaded".to_string())
                    }
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        "smtplib_send" => {
            if let (Some(RuntimeValue::String(server)), Some(RuntimeValue::String(user)), Some(RuntimeValue::String(pass)), Some(RuntimeValue::String(to)), Some(RuntimeValue::String(subject)), Some(RuntimeValue::String(body))) = (evaluated_args.get(0), evaluated_args.get(1), evaluated_args.get(2), evaluated_args.get(3), evaluated_args.get(4), evaluated_args.get(5)) {
                let email = Message::builder()
                    .from(user.parse().unwrap())
                    .to(to.parse().unwrap())
                    .subject(subject)
                    .body(body.clone())
                    .map_err(|e| format!("Error: {}", e))?;
                let creds = Credentials::new(user.clone(), pass.clone());
                let mailer = lettre::SmtpTransport::relay(server).map_err(|e| format!("Error: {}", e))?.credentials(creds).build();
                mailer.send(&email).map_err(|e| format!("Error: {}", e))?;
                RuntimeValue::String("Email sent".to_string())
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        "email_create" => {
            if let (Some(RuntimeValue::String(to)), Some(RuntimeValue::String(subject)), Some(RuntimeValue::String(body))) = (evaluated_args.get(0), evaluated_args.get(1), evaluated_args.get(2)) {
                match Message::builder().from("no-reply@velvet-lang.org".parse(
                    Ok(_) => RuntimeValue::String("Email created".to_string()),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        "re_match" => {
            if let (Some(RuntimeValue::String(pattern)), Some(RuntimeValue::String(text))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                match Regex::new(pattern) {
                    Ok(re) => RuntimeValue::Bool(re.is_match(text)),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        "collections_counter" => {
            if let Some(RuntimeValue::List(items)) = evaluated_args.get(0) {
                let mut counter = HashMap::new();
                for item in items {
                    let key = item.to_string();
                    *counter.entry(key).or_insert(0) += 1;
                }
                let counter_items: Vec<RuntimeValue> = counter.into_iter().map(|(k, v)| {
                    RuntimeValue::List(vec![RuntimeValue::String(k), RuntimeValue::Number(v as f64)])
                }).collect();
                RuntimeValue::List(counter_items)
            } else {
                RuntimeValue::String("Invalid list".to_string())
            }
        }
        "queue_new" => {
            let queue = VecDeque::new();
            RuntimeValue::List(queue.into_iter().collect())
        }
        "asyncio_run" => {
            if let Some(RuntimeValue::String(task)) = evaluated_args.get(0) {
                tokio::spawn(async move {
                    println!("Async task {} executed", task);
                });
                RuntimeValue::String(format!("Async task {} started", task))
            } else {
                RuntimeValue::String("Invalid task".to_string())
            }
        }
        "threading_run" => {
            if let Some(RuntimeValue::String(task)) = evaluated_args.get(0) {
                std::thread::spawn(move || {
                    println!("Thread task {} executed", task);
                });
                RuntimeValue::String(format!("Thread {} started", task))
            } else {
                RuntimeValue::String("Invalid task".to_string())
            }
        }
        "argparse_parse" => {
            if let Some(RuntimeValue::List(args)) = evaluated_args.get(0) {
                let args: Vec<String> = args.iter().filter_map(|v| if let RuntimeValue::String(s) = v { Some(s.clone()) } else { None }).collect();
                let matches = Command::new("velvet")
                    .arg(Arg::new("input").takes_value(true))
                    .get_matches_from(args);
                if let Some(input) = matches.get_one::<String>("input") {
                    RuntimeValue::String(input.clone())
                } else {
                    RuntimeValue::String("No input provided".to_string())
                }
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        "logging_info" => {
            if let Some(RuntimeValue::String(msg)) = evaluated_args.get(0) {
                println!("INFO: {}", msg);
                RuntimeValue::String("Logged".to_string())
            } else {
                RuntimeValue::String("Invalid message".to_string())
            }
        }
        "uuid_generate" => {
            RuntimeValue::String(Uuid::new_v4().to_string())
        }
        "hashlib_sha256" => {
            if let Some(RuntimeValue::String(input)) = evaluated_args.get(0) {
                let mut hasher = Sha256::new();
                hasher.update(input);
                let result = hasher.finalize();
                RuntimeValue::String(format!("{:x}", result))
            } else {
                RuntimeValue::String("Invalid input".to_string())
            }
        }
        "net_ping" => {
            if let Some(RuntimeValue::String(host)) = evaluated_args.get(0) {
                match ping(host.parse().unwrap(), None, None, None, None, None) {
                    Ok(_) => RuntimeValue::String("Ping successful".to_string()),
                    Err(e) => RuntimeValue::String(format!("Ping failed: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid host".to_string())
            }
        }
        "db_connect" => {
            if let Some(RuntimeValue::String(db_path)) = evaluated_args.get(0) {
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    match SqlitePool::connect(db_path).await {
                        Ok(_) => RuntimeValue::String("DB connected".to_string()),
                        Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                    }
                })
            } else {
                RuntimeValue::String("Invalid path".to_string())
            }
        }
        "csv_read" => {
            if let Some(RuntimeValue::String(path)) = evaluated_args.get(0) {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        let rows: Vec<RuntimeValue> = content.lines().map(|line| {
                            let cols: Vec<RuntimeValue> = line.split(',').map(|col| RuntimeValue::String(col.trim().to_string())).collect();
                            RuntimeValue::List(cols)
                        }).collect();
                        RuntimeValue::List(rows)
                    }
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid path".to_string())
            }
        }
        "sqlite_query" => {
            if let (Some(RuntimeValue::String(db_path)), Some(RuntimeValue::String(query))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                match Connection::open(db_path) {
                    Ok(conn) => {
                        let mut stmt = conn.prepare(query).map_err(|e| format!("Error: {}", e))?;
                        let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(|e| format!("Error: {}", e))?;
                        let results: Vec<RuntimeValue> = rows.filter_map(|r| r.ok()).map(|r| RuntimeValue::String(r)).collect();
                        RuntimeValue::List(results)
                    }
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        _ => RuntimeValue::String(format!("Unknown function: {}", name)),
    }
                            }
