use std::time;

const SINCE: time::SystemTime = time::UNIX_EPOCH;

pub struct Consumer<C> {
    channel: C,
    last_activity: time::Duration,
}

fn x() -> time::Duration {
    return time::SystemTime::now().duration_since(SINCE).unwrap();
}

pub fn create<C>(channel: C) -> Consumer<C> {
    Consumer {
        channel: channel,
        last_activity: x(),
    }
}
