#[macro_use]
extern crate log;

use wasi::{
    fd_close, fd_read, poll_oneoff, sock_accept, Event, Iovec, Subscription,
    SubscriptionFdReadwrite, SubscriptionU, SubscriptionUU, ERRNO_AGAIN, FDFLAGS_NONBLOCK,
};

fn main() {
    let mut subscription = vec![Subscription {
        userdata: 1,
        u: SubscriptionU {
            tag: 1,
            u: SubscriptionUU {
                fd_read: SubscriptionFdReadwrite { file_descriptor: 3 },
            },
        },
    }];

    let mut events = vec![unsafe { std::mem::zeroed::<Event>() }; 1];

    loop {
        debug!("[0.0] Looping");

        let num = unsafe {
            poll_oneoff(subscription.as_ptr(), events.as_mut_ptr(), events.len()).unwrap()
        };

        debug!("[1.0] Polled ({} events)", num);

        for i in 0..num {
            let event = &events[i];

            debug!(
                "[2.0] Handling event `{}` of type `{}`",
                event.userdata,
                event.type_.raw()
            );

            match event.userdata {
                1 => accept_connection(&mut subscription),
                _ => handle_read(event, &mut subscription),
            }
        }

        events = vec![unsafe { std::mem::zeroed::<Event>() }; subscription.len()];
    }
}

fn accept_connection(subscription: &mut Vec<Subscription>) {
    debug!("[3.0] Attempting to accept connection");

    let file_descriptor = unsafe {
        match sock_accept(3, FDFLAGS_NONBLOCK) {
            Ok(n) => n,
            Err(_) => return,
        }
    };

    debug!("[3.1] Accepted connection. FD = {}", file_descriptor);

    subscription.push(Subscription {
        userdata: file_descriptor as u64,
        u: SubscriptionU {
            tag: 1,
            u: SubscriptionUU {
                fd_read: SubscriptionFdReadwrite { file_descriptor },
            },
        },
    });
}

fn handle_read(event: &Event, subscription: &mut Vec<Subscription>) {
    if event.fd_readwrite.nbytes == 0 {
        debug!("[4.2] Removing socket {}", event.userdata);
        subscription.retain(|x| x.userdata != event.userdata);

        unsafe {
            fd_close(event.userdata as u32).unwrap();
        }

        return;
    }

    let mut buf = vec![0u8; event.fd_readwrite.nbytes as usize];
    let iovec = [Iovec {
        buf: buf.as_mut_ptr(),
        buf_len: buf.len(),
    }];

    debug!(
        "[4.0] Attempting to read {} bytes from socket {}",
        event.fd_readwrite.nbytes, event.userdata
    );

    let num = unsafe {
        match fd_read(event.userdata as u32, &iovec) {
            Ok(n) => n,
            Err(e) => {
                if e != ERRNO_AGAIN {
                    debug!(
                        "[4.2] Removing socket {} due to error `{:?}`",
                        event.userdata, e
                    );

                    subscription.retain(|x| x.userdata != event.userdata);

                    fd_close(event.userdata as u32).unwrap();
                }

                return;
            }
        }
    };

    println!(
        "[4.1] Socket {}: {}",
        event.userdata,
        String::from_utf8_lossy(&buf[..num]).trim(),
    );
}
