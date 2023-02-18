/*!
<https://github.com/grpc/grpc-go/blob/master/internal/grpcsync/event.go>
*/

use std::sync::{atomic, Once};

/// Event represents a one-time event that may occur in the future.
pub struct Event {
    fired: i32,
    c: chan struct{},
    o: Once
}

impl Event {
    /// Returns a new, ready-to-use Event.
    pub fn new() -> Self {
        Self { c: make(chan struct{}) }
    }

    /**
    Causes e to complete.  It is safe to call multiple times, and concurrently.  It returns true iff this call to fire caused the signaling channel returned by Done to close.
    */
    pub fn fire(&self) -> bool {
        let mut ret = false;
        self.o.Do(|| {
            atomic.StoreInt32(&self.fired, 1);
            close(self.c);
            ret = true;
        });
        ret
    }

    /// Returns a channel that will be closed when fire is called.
    pub fn done(&self) <-chan struct{} {
        self.c
    }

    /// Returns true if fire has been called.
    pub fn has_fired(&self) -> bool {
        atomic.LoadInt32(&self.fired) == 1
    }
}
