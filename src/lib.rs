#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod service;

include!(concat!(env!("OUT_DIR"), "/_.rs"));

pub struct MyExampleContext {
  pub hardcoded_database: Vec<Book>,
}

// use std::sync::Arc;

use dcl_rpc::{client::RpcClient, server::RpcServer, transports::memory::MemoryTransport};
use napi::tokio::{select, sync::oneshot};
use service::book_service;

#[napi]
pub async fn execute_rpc_rust() {
  let (t_client, server_t) = MemoryTransport::create();

  let ctx = MyExampleContext {
    hardcoded_database: vec![Book {
      isbn: 1000,
      title: "AA".to_string(),
      author: "me".to_string(),
    }],
  };

  let mut server: RpcServer<MyExampleContext, _> = RpcServer::create(ctx);

  server.set_module_registrator_handler(|port: &mut RpcServerPort<MyExampleContext>| {
    BookServiceRegistration::register_service(port, book_service::BookService {})
  });

  match server.attach_transport(Arc::new(server_t)).await {
    Ok(_) => {
      println!("> RpcServer > first transport attached successfully");
    }
    Err(_) => {
      println!("> RpcServer > unable to attach transport");
      panic!()
    }
  }

  let mut client = RpcClient::new(t_client).await.unwrap();

  let s_handle = napi::tokio::spawn(async move { server.run().await });

  let (c_sen, c_rev) = oneshot::channel();

  let c_handle = napi::tokio::spawn(async move {
    let port = client.create_port("A").await.unwrap();
    println!("port {}", port.port_id());
    let res = port
      .load_module::<BookServiceClient<MemoryTransport>>("BookService")
      .await
      .unwrap();

    let book = res.get_book(GetBookRequest { isbn: 1000 }).await.unwrap();
    println!("book {}", book.title);
    c_sen.send(true).unwrap();
  });

  c_handle.await.unwrap();
  select! {
    _ = c_rev => {
      println!("Example finished manually")
    },
    _ =  s_handle => {
        println!("Server terminated unexpectedly")
    }
  }
}
