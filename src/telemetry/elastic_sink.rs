use std::sync::{Arc};
use std::net::SocketAddr;
use std::time::Duration;

use tokio::io::AsyncWriteExt;
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        RwLock,
    },
    task::JoinHandle,
};

use tokio::net::TcpSocket;

use crate::app::AppContext;

/* lazy_static::lazy_static! {
    pub static ref SINK: ElasticSink = ElasticSink::new();
} */

pub struct ElasticSink {
    buffer: Arc<RwLock<Vec<Vec<u8>>>>,
    sender: UnboundedSender<Vec<u8>>,
    log_writer: JoinHandle<()>,
    log_flusher: JoinHandle<()>,
}

pub struct ElasticWriter {
    sender: UnboundedSender<Vec<u8>>,
}

impl ElasticSink {
    pub fn new(log_stash_url: SocketAddr, app_context: Arc<AppContext>) -> Self {
        let (sender, recv) = tokio::sync::mpsc::unbounded_channel();

        let buffer = Arc::new(RwLock::new(vec![]));
        let log_flusher = tokio::spawn(log_flusher_thread(
            log_stash_url,
            buffer.clone(),
            app_context.clone(),
        ));
        let log_writer = tokio::spawn(log_writer_thread(recv, buffer.clone()));
        let res = Self {
            buffer: buffer,
            sender: sender.clone(),
            log_flusher: log_flusher,
            log_writer: log_writer,
        };

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
async fn log_flusher_thread(
    log_stash_url: SocketAddr,
    data: Arc<RwLock<Vec<Vec<u8>>>>,
    app_context: Arc<AppContext>,
) {
    let mut stream = connect_to_socket(log_stash_url).await;

    loop {
        let mut write_access = data.as_ref().write().await;
        while let Some(res) = write_access.pop() {
            let send_res = stream.write(&res).await;
            match send_res {
                Ok(size) => {
                    println!("Send logs {:?}", size)
                }
                Err(err) => println!("Can't write logs to logstash server {:?}", err),
            }
        }

        if app_context.is_shutting_down() {
            return;
        }

        tokio::time::sleep(Duration::from_millis(250)).await
    }
}

async fn connect_to_socket(log_stash_url: SocketAddr) -> TcpStream {
    let socket: TcpSocket;
    let socket_result = TcpSocket::new_v4();
    match socket_result {
        Ok(x) => socket = x,
        Err(err) => {
            println!("Can't create socket for logs {:?}", err);
            panic!("Can't create socket for logs {:?}", err);
        }
    }
    let connect_res = socket.connect(log_stash_url).await;
    let stream: TcpStream;
    match connect_res {
        Ok(x) => {
            stream = x;
        }
        Err(err) => {
            println!("Can't connect to logstash server {:?}", err);
            panic!("Can't connect to logstash server {:?}", err);
        }
    }
    stream
}
