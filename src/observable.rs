use std::sync::{Arc, Mutex, Weak};
use observer::{Observer, MutObserver};

pub type ObserverRef = Weak<Mutex<Observer + Sync + Send>>;
pub type MutObserverRef = Weak<Mutex<MutObserver + Sync + Send>>;

pub trait Observable {
    fn register(&mut self, observer: ObserverRef);
}

pub trait MutObservable {
    fn register_mut(&mut self, observer: MutObserverRef);
}
