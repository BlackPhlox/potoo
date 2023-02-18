use std::{future::Future, task::Poll, thread};

use tokio::{net::TcpListener, runtime::Builder};

pub struct PotooServer {
    ip_port: String,
}

impl Default for PotooServer {
    fn default() -> Self {
        PotooServer {
            ip_port: String::from("127.0.0.1:7878"),
        }
    }
}

impl PotooServer {
    pub fn start(self) {
        thread::spawn(move || {
            Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    self.start_async().await;
                });
        });
    }

    pub async fn start_async(self) -> () {
        let listener = TcpListener::bind(&self.ip_port).await.unwrap();
        println!("Started server on {}", self.ip_port,);
        let _r = mini_redis::server::run(listener, tokio::signal::ctrl_c()).await;
    }
}
