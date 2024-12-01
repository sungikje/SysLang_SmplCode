use std::net::TcpListener;
use std::io::Read;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind");

  for stream in listener.incoming() {
    println!("Waiting for packet...");
    let mut stream = stream.unwrap();
    let mut buffer = [0; 1024];

    // 실제로 읽은 바이트 수를 확인
    let bytes_read = stream.read(&mut buffer).unwrap();

    // 읽은 데이터의 유효 부분만 사용
    let command = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("packet: {}", command.trim());

    if command.trim() == "exit" {
      println!("Exit command received. Shutting down...");
      break; // 서버 종료
    }
  }
  println!("Server has stopped.");
}
