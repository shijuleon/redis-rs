use std::io::{Read, Write};
use std::net::TcpStream;

fn serialize_bulk_array(commands: &[&str]) -> String {
  let len = commands.len();

  let mut msg = String::from(format!("*{}", len));
  for command in commands {
    msg.push_str("\r\n");
    msg.push_str("$");
    msg.push_str(&command.len().to_string());
    msg.push_str("\r\n");
    msg.push_str(command)
  }
  msg.push_str("\r\n");
  msg
}

fn send_command(command: &str) -> String {
  let mut response = String::new();
  match TcpStream::connect("localhost:6379") {
    Ok(mut stream) => {
      let mut msg = [0 as u8; 512];
      write!(&mut msg[..], "{}", command);
      stream.write(&msg[..]).unwrap();
      let mut client_buffer = [0u8; 512];

      loop {
        match stream.read(&mut client_buffer) {
          Ok(n) => match String::from_utf8(client_buffer[..n].to_vec()) {
            Ok(res) => {
              response.push_str(&res);
              // use proper redis EOF
              if n < 512 {
                return response;
              }
            },
            Err(e) => println!("Error creating string from array: {e}"),
          },
          Err(e) => println!("Error reading stream: {}", e),
        }
      }
    }
    Err(e) => {
      println!("Failed to connect: {}", e);
    }
  }

  return response;
}

fn get(key: &str) -> String {
  return send_command(&serialize_bulk_array(&["GET", &key]));
}

fn zrevrangebyscore_limit(key: &str, min: &str, max: &str, offset: u8, limit: u64) -> String {
  return send_command(&serialize_bulk_array(&["ZREVRANGEBYSCORE", &key, min, max, "LIMIT", &(offset.to_string()), &(limit.to_string())]));
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::{Duration, Instant};

  #[test]
  fn it_works() {
    let mut time_taken_redis = Duration::new(0, 0);
    let start = Instant::now();

    let response = zrevrangebyscore_limit("newsfeed:scores", "inf", "-inf", 0, 1000);
    let response_as_lines = response.split("\n");

    let mut feeds_list = Vec::new();

    for line in response_as_lines {
      if !(line.get(..1) == Some("$")) {
        feeds_list.push(line.trim());
      }
    }

    // newline
    feeds_list.pop();

    for feed in feeds_list {
      get(&format!("{}:articles:recent", feed));
      //println!("{}", get(&format!("{}:articles:recent", feed)));
    }

    let end = Instant::now();
    time_taken_redis = time_taken_redis + end.duration_since(start);
    println!("redis ops took {} ms", time_taken_redis.as_millis());
  }
}
