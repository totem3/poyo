use event::Event;

pub trait Observer {
    fn notify(&self, &Event);
}

pub trait MutObserver {
    fn notify_mut(&mut self, &Event);
}
