pub trait Actor: Send + 'static {
    type Message: Send + 'static;

    fn handle_message(&mut self, msg: Self::Message);

    fn run(mut self, mut receiver: tokio::sync::mpsc::Receiver<Self::Message>)
    where
        Self: Sized,
    {
        while let Some(msg) = receiver.blocking_recv() {
            self.handle_message(msg);
        }
    }
}
