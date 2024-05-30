use std::cell::RefCell;
use std::fmt;
#[cfg(feature = "futures")]
use std::future::Future;
use std::rc::Rc;

use serde::de::Deserialize;
use serde::ser::Serialize;
use wasm_bindgen_futures::spawn_local;

use super::handler_id::HandlerId;
use super::lifecycle::{WorkerLifecycleEvent, WorkerRunnable, WorkerState};
use super::messages::FromWorker;
use super::native_worker::{DedicatedWorker, NativeWorkerExt, WorkerSelf};
use super::traits::Worker;
use super::Shared;
use crate::codec::Codec;

/// A handle that closes the worker when it is dropped.
pub struct WorkerDestroyHandle<W>
where
    W: Worker + 'static,
{
    scope: WorkerScope<W>,
}

impl<W: Worker> fmt::Debug for WorkerDestroyHandle<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WorkerDestroyHandle<_>")
    }
}

impl<W> WorkerDestroyHandle<W>
where
    W: Worker,
{
    pub(crate) fn new(scope: WorkerScope<W>) -> Self {
        Self { scope }
    }
}

impl<W> Drop for WorkerDestroyHandle<W>
where
    W: Worker,
{
    fn drop(&mut self) {
        self.scope.send(WorkerLifecycleEvent::Destroy);
    }
}

/// This struct holds a reference to a component and to a global scheduler.
pub struct WorkerScope<W: Worker> {
    state: Shared<WorkerState<W>>,
    post_msg: Rc<dyn Fn(FromWorker<W>)>,
}

impl<W: Worker> fmt::Debug for WorkerScope<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WorkerScope<_>")
    }
}

impl<W: Worker> Clone for WorkerScope<W> {
    fn clone(&self) -> Self {
        WorkerScope {
            state: self.state.clone(),
            post_msg: self.post_msg.clone(),
        }
    }
}

impl<W> WorkerScope<W>
where
    W: Worker + 'static,
{
    /// Create worker scope
    pub(crate) fn new<CODEC>() -> Self
    where
        CODEC: Codec,
        W::Output: Serialize + for<'de> Deserialize<'de>,
    {
        let post_msg = move |msg: FromWorker<W>| {
            DedicatedWorker::worker_self().post_packed_message::<_, CODEC>(msg)
        };

        let state = Rc::new(RefCell::new(WorkerState::new()));
        WorkerScope {
            post_msg: Rc::new(post_msg),
            state,
        }
    }

    /// Schedule message for sending to worker
    pub(crate) fn send(&self, event: WorkerLifecycleEvent<W>) {
        let state = self.state.clone();

        // We can implement a custom scheduler,
        // but it's easier to borrow the one from wasm-bindgen-futures.
        spawn_local(async move {
            WorkerRunnable { state, event }.run();
        });
    }

    /// Send response to a worker bridge.
    pub fn respond(&self, id: HandlerId, output: W::Output) {
        let msg = FromWorker::<W>::ProcessOutput(id, output);
        (self.post_msg)(msg);
    }

    /// Send a message to the worker
    pub fn send_message<T>(&self, msg: T)
    where
        T: Into<W::Message>,
    {
        self.send(WorkerLifecycleEvent::Message(msg.into()));
    }

    /// Create a callback which will send a message to the worker when invoked.
    pub fn callback<F, IN, M>(&self, function: F) -> Rc<dyn Fn(IN)>
    where
        M: Into<W::Message>,
        F: Fn(IN) -> M + 'static,
    {
        let scope = self.clone();
        let closure = move |input| {
            let output = function(input).into();
            scope.send(WorkerLifecycleEvent::Message(output));
        };
        Rc::new(closure)
    }

    /// This method creates a callback which returns a Future which
    /// returns a message to be sent back to the worker
    ///
    /// # Panics
    /// If the future panics, then the promise will not resolve, and
    /// will leak.
    #[cfg(feature = "futures")]
    #[cfg_attr(docsrs, doc(cfg(feature = "futures")))]
    pub fn callback_future<FN, FU, IN, M>(&self, function: FN) ->
