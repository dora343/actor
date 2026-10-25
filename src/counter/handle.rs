use super::actor::CounterActor;
use crate::actor::Actor;
use crate::counter::message::CounterMessage;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Clone)]
pub struct CounterHandle {
    sender: mpsc::Sender<CounterMessage>,
}

impl CounterHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = CounterActor::new();
        std::thread::spawn(move || actor.run(receiver));
        Self { sender }
    }

    pub async fn get_count(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = CounterMessage::GetCount { respond_to: send };
        let _ = self
            .sender
            .send(msg)
            .await
            .expect("Failed to send message to actor");

        recv.await.expect("Actor task has been killed")
    }

    pub async fn set_count(&self, count: u32) {
        let msg = CounterMessage::SetCount { count };
        let _ = self
            .sender
            .send(msg)
            .await
            .expect("Failed to send message to actor");
    }

    pub async fn reset_count(&self) {
        let msg = CounterMessage::ResetCount;
        let _ = self
            .sender
            .send(msg)
            .await
            .expect("Failed to send message to actor");
    }
}
