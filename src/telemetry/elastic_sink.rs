use std::{
    io::Write,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::{sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    RwLock,
}, io::AsyncWriteExt, net::TcpStream};

use tokio::net::TcpSocket;

/* lazy_static::lazy_static! {
    pub static ref SINK: ElasticSink = ElasticSink::new();
} */

pub struct ElasticSink {
    buffer: Arc<RwLock<Vec<Vec<u8>>>>,
    sender: UnboundedSender<Vec<u8>>,
}

pub struct ElasticWriter {
    sender: UnboundedSender<Vec<u8>>,
}

impl ElasticSink {
    pub fn new(log_stash_url: SocketAddr) -> Self {
        let (sender, recv) = tokio::sync::mpsc::unbounded_channel();

        let res = Self {
            buffer: Arc::new(RwLock::new(vec![])),
            sender: sender.clone(),
        };

        tokio::spawn(log_writer_thread(recv, res.buffer.clone()));
        tokio::spawn(log_flusher_thread(log_stash_url, res.buffer.clone()));

        res
    }

    pub fn create_writer(&self) -> ElasticWriter {
        ElasticWriter {
            sender: self.sender.clone(),
        }
    }
}

impl std::io::Write for ElasticWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::stdout().write_all(&buf).unwrap();
        self.sender.send(buf.to_vec()).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::Result::Ok(())
    }
}

async fn log_writer_thread(mut recv: UnboundedReceiver<Vec<u8>>, data: Arc<RwLock<Vec<Vec<u8>>>>) {
    while let Some(next_item) = recv.recv().await {
        let mut write_access = data.as_ref().write().await;
        write_access.push(next_item);
    }
}

//Executes each second
async fn log_flusher_thread(log_stash_url: SocketAddr, data: Arc<RwLock<Vec<Vec<u8>>>>) {
    let mut stream = connect_to_socket(log_stash_url).await;

    loop {
        let mut write_access = data.as_ref().write().await;
        while let Some(res) = write_access.pop() {
            
            let send_res = stream.write(&res).await;
            match send_res {
                Ok(size) => {println!("Send logs {:?}", size)},
                Err(err) => println!("Can't write logs to logstash server {:?}", err),
            }
        }

        tokio::time::sleep(Duration::from_millis(250)).await
    }
}

async fn connect_to_socket(log_stash_url: SocketAddr) -> TcpStream {
    let socket:TcpSocket;
    let socket_result = TcpSocket::new_v4();
    match socket_result {
        Ok(x) => {socket = x},
        Err(err) => {
            println!("Can't create socket for logs {:?}", err);
            panic!("Can't create socket for logs {:?}", err);
        },
    }
    let connect_res = socket.connect(log_stash_url).await;
    let mut stream:TcpStream;
    match connect_res {
        Ok(x) => {stream = x;},
        Err(err) => {
            println!("Can't connect to logstash server {:?}", err);
            panic!("Can't connect to logstash server {:?}", err);
        },
    }
    stream
}
