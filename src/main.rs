use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;


// 客户端消息处理方法
fn handle_client(mut stream: TcpStream){
    // 定义一个存储用的数组
    let mut buf = [0; 1024];
    // 循环处理客户端消息
    for _ in 0..1000 {
        // 读取客户端消息
        let bytes_read = stream.read(&mut buf).unwrap();
        // 打印接收到的信息
        println!("Request: {}", String::from_utf8_lossy(&buf[..bytes_read]));
        // 返回输入的消息
        stream.write(&buf[..bytes_read]).unwrap();
    }
}


fn main() {
    // 定义要绑定的ip和port
    let addr = "127.0.0.1:9000";
    // 创建一个tcp监听, 并绑定ip和port
    let listener = TcpListener::bind(addr).unwrap();
    // 打印服务器监听信息
    println!("Server listening on port {}",addr);
    // 接收客户端的连接信息，并启动新的线程处理每个客户端连接
    for stream in listener.incoming() {
        // 模式匹配
        match stream {
            // 连接正常
            Ok(stream) => {
                // 打印连接信息
                println!("New client connection: {}", stream.peer_addr().unwrap());
                // 如果连接成功，启动新的线程处理
                thread::spawn(move || {
                    handle_client(stream)
               });
            }
            // 连接出错
            Err(e) => { 
                // 打印错误信息
                println!("Error: {}", e);
             }
        }
    }
    // 关闭Tcp监听链接
    drop(listener); 
}
