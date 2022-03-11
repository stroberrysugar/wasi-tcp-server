use wasi::{
    fd_read, poll_oneoff, sock_accept, Event, Iovec, Subscription, SubscriptionFdReadwrite,
    SubscriptionU, SubscriptionUU,
};

fn main() {
    let connection = unsafe {
        loop {
            match sock_accept(3, 0) {
                Ok(n) => break n,
                Err(_) => {}
            }
        }
    };

    let subscription = [Subscription {
        userdata: 1,
        u: SubscriptionU {
            tag: 1,
            u: SubscriptionUU {
                fd_read: SubscriptionFdReadwrite {
                    file_descriptor: connection,
                },
            },
        },
    }];

    let mut events = unsafe { std::mem::zeroed::<[Event; 1]>() };

    unsafe {
        assert_eq!(
            poll_oneoff(subscription.as_ptr(), events.as_mut_ptr(), 1).unwrap(),
            1
        );
    }

    println!("{:?}", events[0].type_);

    let mut buf = [0u8; 4];
    let iovec = [Iovec {
        buf: buf.as_mut_ptr(),
        buf_len: buf.len(),
    }];

    unsafe {
        fd_read(connection, &iovec).unwrap();
    }

    println!("Hello, world! {}", String::from_utf8_lossy(&buf));
}
