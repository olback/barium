use {
    padlock,
    std::{thread, time::Duration, sync::{Arc, Mutex}},
    glib::{clone, Sender},
    crate::consts::KEEP_ALIVE_INTERVAL,
    user_idle::UserIdle,
    log::debug
};

#[derive(Debug, Clone)]
pub struct IdleTracker {
    senders: Arc<Mutex<Vec<Sender<u64>>>>
}

impl IdleTracker {

    pub fn new() -> Self {

        let inner = Self {
            senders: Arc::new(Mutex::new(Vec::new()))
        };

        thread::spawn(clone!(@strong inner.senders as senders => move || {

            loop {

                if let Ok(afk_time) = UserIdle::get_time() {

                    debug!("Idle for {} seconds", afk_time.as_seconds());

                    padlock::mutex_lock(&senders, |lock| {

                        for i in 0..lock.len() {
                            match lock[i].send(afk_time.as_seconds()) {
                                Ok(_) => {},
                                Err(_) => {
                                    debug!("Dropping IdleTracker");
                                    lock.remove(i);
                                }
                            }
                        }

                    });

                }

                thread::sleep(Duration::from_secs(KEEP_ALIVE_INTERVAL))

            }

        }));

        inner

    }

    pub fn add(&self, tx: Sender<u64>) {

        debug!("Adding IdleTracker");

        padlock::mutex_lock(&self.senders, |lock| {
            lock.push(tx);
        });

    }

}
