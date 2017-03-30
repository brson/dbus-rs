extern crate dbus;
extern crate tokio_core;
extern crate futures;

use tokio_core::reactor::Core;
use dbus::{Connection, BusType};
use dbus::tokio::TokioConnection;
use futures::Future;

fn main() {
    let mut core = Core::new().unwrap();
    let dbus = Connection::get_private(BusType::Session).unwrap();
    let dbus = TokioConnection::new(dbus, core.handle());

    let future = dbus.call("org.freedesktop.DBus",
                           "/",
                           "org.freedesktop.DBus",
                           "ListNames")
        .unwrap()
        .and_then(|vec: Vec<String>| {
            println!("done");
            for name in vec {
                println!("{:?}", name);
            }
            Ok(())
        });

    core.run(future).unwrap();
}
