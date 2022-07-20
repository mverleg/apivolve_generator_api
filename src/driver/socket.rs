use ::std::net::TcpListener;
use ::std::thread;

use ::log::debug;
use ::log::error;

use crate::gen1::ErrMsg;

fn accept_single_connection() -> Result<(), ErrMsg> {
    let address = "127.0.0.1:47400";
    let listener = TcpListener::bind(address)
        .map_err(|err| format!("failed to listen for tcp connection on {}, err {}", address, err))?;
    // let mut worker;
    if let Some(connection) = listener.incoming().next() {
        match connection {
            Ok(connection) => {}
            Err(err) => panic!("failed to accept the first connection on {}, err {}", address, err),
        }
    }
    thread::spawn(|| reject_extra_connections(listener));
    // for connection in listener.incoming() {
    //     let stream = connection.map_err(|err| format!("failed to accept the first connection on {}, err {}", address, err))?;
    //     worker = thread::spawn(move || stream);
    // }
    // worker.join().expect("generator thread failed to join");
    Ok(())
}

fn reject_extra_connections(listener: TcpListener) {
    for extra_connection in listener.incoming() {
        debug!("unexpected connection: {:?}", extra_connection);
        error!("received more than one connection! this should not happen, it will be ignored");
    }
}
