use crossbeam_channel;
pub use crossbeam_channel::unbounded;

pub trait SenderReturn<T, R> {
    fn send(&self, t: T) -> Result<R, crossbeam_channel::RecvError>;
}

impl<T, R> SenderReturn<T, R> for crossbeam_channel::Sender<(crossbeam_channel::Sender<R>, T)> {
    fn send(&self, t: T) -> Result<R, crossbeam_channel::RecvError> {
        let (sender, receiver) = crossbeam_channel::unbounded::<R>();

        if crossbeam_channel::Sender::send(&self, (sender, t)).is_err() {
            print!("ReceiverReturn was dropped too early");
        }

        receiver.recv()
    }
}

pub trait ReceiverReturn<T, R> {
    fn recv<F>(&self, func: F) -> Result<(), crossbeam_channel::RecvError> where F: Fn(T) -> R;
}

impl <T, R> ReceiverReturn<T, R> for crossbeam_channel::Receiver<(crossbeam_channel::Sender<R>, T)> {
    fn recv<F>(&self, func: F) -> Result<(), crossbeam_channel::RecvError> where F: Fn(T) -> R {
        match crossbeam_channel::Receiver::recv(&self) {
            Ok((sender, t)) => {
                sender.send(func(t)).unwrap();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn multiply_itself() {
        let (s,r) = unbounded();

        thread::spawn(move || {
            let r = r;
            while ReceiverReturn::recv(&r, |x: u32| x*x).is_ok() {

            }
        });

        assert_eq!(SenderReturn::send(&s,1), Ok(1));
        assert_eq!(SenderReturn::send(&s,2), Ok(4));
        assert_eq!(SenderReturn::send(&s,3), Ok(9));
        assert_eq!(SenderReturn::send(&s,4), Ok(16));

    }

    #[test]
    fn api_v2() {
        use plugin_api_v2::{Request,Reply};

        let (s,r) = unbounded();

        thread::spawn(move || {
            let r = r;
            loop {
                let result = ReceiverReturn::recv(&r, |request: Request| {
                    match request {
                        Request::ApiToken => Reply::ApiToken(String::from("zzzz-xxxxxxxxxxxx-yyyyyyyyyyyyyyyyyyyyyyyy")),
                        _ => Reply::NotConfigured,
                    }
                });
                if result.is_err() {
                    break;
                }
            }
        });

        assert_eq!(SenderReturn::send(&s, Request::ApiToken), Ok(Reply::ApiToken(String::from("zzzz-xxxxxxxxxxxx-yyyyyyyyyyyyyyyyyyyyyyyy"))));
        assert_eq!(SenderReturn::send(&s, Request::AdminApiToken), Ok(Reply::NotConfigured));
    }

    #[test]
    fn api_v3() {
        use plugin_api_v2::{Request,Reply};
        use crossbeam_channel;

        let (s,r) = unbounded::<(crossbeam_channel::Sender<Reply>, Request)>();

        drop(r);
        let _ = SenderReturn::send(&s, Request::AdminApiToken);
    }
}