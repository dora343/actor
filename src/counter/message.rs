use tokio::sync::oneshot;


pub(super) enum CounterMessage {
    GetCount { respond_to: oneshot::Sender<u32> },
    SetCount { count: u32 },
    ResetCount,
}
