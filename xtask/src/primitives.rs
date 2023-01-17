/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use std::collections::HashMap;
use tokio::sync::{broadcast, mpsc, oneshot};

///
pub type Bundle<T = String> = HashMap<T, Vec<Vec<T>>>;

///
pub type TokioChannelBroadcast<T> = (broadcast::Sender<T>, broadcast::Receiver<T>);
///
pub type TokioChannelMPSC<T> = (mpsc::Sender<T>, mpsc::Receiver<T>);
///
pub type TokioChannelUnboundedMPSC<T> = (mpsc::UnboundedSender<T>, mpsc::UnboundedReceiver<T>);

///
pub type TokioChannelOneshot<T> = (oneshot::Sender<T>, oneshot::Receiver<T>);

#[derive(Debug)]
pub enum Channels<T: Clone> {
    Broadcast(TokioChannelBroadcast<T>),
    MPSC(TokioChannelMPSC<T>),
    Oneshot(TokioChannelOneshot<T>),
    UnboundedMPSC(TokioChannelUnboundedMPSC<T>),
}

impl<T: Clone> Channels<T> {
    pub fn broadcast(capacity: usize) -> Self {
        let cp = broadcast::channel(capacity);
        Self::Broadcast(cp)
    }
    pub fn mpsc(capacity: usize) -> Self {
        let cp = mpsc::channel(capacity);
        Self::MPSC(cp)
    }
    pub fn oneshot() -> Self {
        let cp = oneshot::channel();
        Self::Oneshot(cp)
    }
    pub fn unbounded_mpsc() -> Self {
        let cp = mpsc::unbounded_channel();
        Self::UnboundedMPSC(cp)
    }
}
