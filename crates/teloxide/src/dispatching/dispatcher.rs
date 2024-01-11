use crate::{
    dispatching::{
        distribution::default_distribution_function, DefaultKey, DpHandlerDescription,
        ShutdownToken,
    },
    error_handlers::{ErrorHandler, LoggingErrorHandler},
    requests::{Request, Requester},
    types::{Update, UpdateKind},
    update_listeners::{self, UpdateListener},
};

use dptree::di::{DependencyMap, DependencySupplier};
use either::Either;
use futures::{
    future::{self, BoxFuture},
    stream::FuturesUnordered,
    FutureExt as _, StreamExt as _,
};
use tokio_stream::wrappers::ReceiverStream;

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Debug, Display},
    future::Future,
    hash::Hash,
    ops::{ControlFlow, Deref},
    pin::pin,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
};

/// The builder for [`Dispatcher`].
///
/// See also: ["Dispatching or
/// REPLs?"](../dispatching/index.html#dispatching-or-repls)
pub struct DispatcherBuilder<R, Err, Key> {
    bot: R,
    dependencies: DependencyMap,
    handler: Arc<UpdateHandler<Err>>,
    default_handler: DefaultHandler,
    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,
    ctrlc_handler: bool,
    distribution_f: fn(&Update) -> Option<Key>,
    worker_queue_size: usize,
}

impl<R, Err, Key> DispatcherBuilder<R, Err, Key>
where
    R: Clone + Requester + Clone + Send + Sync + 'static,
    Err: Debug + Send + Sync + 'static,
{
    /// Specifies a handler that will be called for an unhandled update.
    ///
    /// By default, it is a mere [`log::warn`].
    #[must_use]
    pub fn default_handler<H, Fut>(self, handler: H) -> Self
    where
        H: Fn(Arc<Update>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let handler = Arc::new(handler);

        Self {
            default_handler: Arc::new(move |upd| {
                let handler = Arc::clone(&handler);
                Box::pin(handler(upd))
            }),
            ..self
        }
    }

    /// Specifies a handler that will be called on a handler error.
    ///
    /// By default, it is [`LoggingErrorHandler`].
    #[must_use]
    pub fn error_handler(self, handler: Arc<dyn ErrorHandler<Err> + Send + Sync>) -> Self {
        Self { error_handler: handler, ..self }
    }

    /// Specifies dependencies that can be used inside of handlers.
    ///
    /// By default, there is no dependencies.
    #[must_use]
    pub fn dependencies(self, dependencies: DependencyMap) -> Self {
        Self { dependencies, ..self }
    }

    /// Enables the `^C` handler that [`shutdown`]s dispatching.
    ///
    /// [`shutdown`]: ShutdownToken::shutdown
    #[cfg(feature = "ctrlc_handler")]
    #[must_use]
    pub fn enable_ctrlc_handler(self) -> Self {
        Self { ctrlc_handler: true, ..self }
    }

    /// Specifies size of the queue for workers.
    ///
    /// By default it's 64.
    #[must_use]
    pub fn worker_queue_size(self, size: usize) -> Self {
        Self { worker_queue_size: size, ..self }
    }

    /// Specifies the distribution function that decides how updates are grouped
    /// before execution.
    #[must_use]
    pub fn distribution_function<K>(
        self,
        f: fn(&Update) -> Option<K>,
    ) -> DispatcherBuilder<R, Err, K>
    where
        K: Hash + Eq,
    {
        let Self {
            bot,
            dependencies,
            handler,
            default_handler,
            error_handler,
            ctrlc_handler,
            distribution_f: _,
            worker_queue_size,
        } = self;

        DispatcherBuilder {
            bot,
            dependencies,
            handler,
            default_handler,
            error_handler,
            ctrlc_handler,
            distribution_f: f,
            worker_queue_size,
        }
    }

    /// Constructs [`Dispatcher`].
    #[must_use]
    pub fn build(self) -> Dispatcher<R, Err, Key> {
        let Self {
            bot,
            dependencies,
            handler,
            default_handler,
            error_handler,
            distribution_f,
            worker_queue_size,
            ctrlc_handler,
        } = self;

        // If the `ctrlc_handler` feature is not enabled, don't emit a warning.
        let _ = ctrlc_handler;

        let dp = Dispatcher {
            bot,
            dependencies,
            handler,
            default_handler,
            error_handler,
            state: ShutdownToken::new(),
            distribution_f,
            worker_queue_size,
            workers: HashMap::new(),
            default_worker: None,
            current_number_of_active_workers: Default::default(),
            max_number_of_active_workers: Default::default(),
        };

        #[cfg(feature = "ctrlc_handler")]
        {
            if ctrlc_handler {
                let mut dp = dp;
                dp.setup_ctrlc_handler_inner();
                return dp;
            }
        }

        dp
    }
}

/// The base for update dispatching.
///
/// Updates from different chats are handled concurrently, whereas updates from
/// the same chats are handled sequentially. If the dispatcher is unable to
/// determine a chat ID of an incoming update, it will be handled concurrently.
/// Note that this behaviour can be altered with [`distribution_function`].
///
/// See also: ["Dispatching or
/// REPLs?"](../dispatching/index.html#dispatching-or-repls)
///
/// [`distribution_function`]: DispatcherBuilder::distribution_function
pub struct Dispatcher<R, Err, Key> {
    bot: R,
    dependencies: DependencyMap,

    handler: Arc<UpdateHandler<Err>>,
    default_handler: DefaultHandler,

    distribution_f: fn(&Update) -> Option<Key>,
    worker_queue_size: usize,
    current_number_of_active_workers: Arc<AtomicU32>,
    max_number_of_active_workers: Arc<AtomicU32>,
    // Tokio TX channel parts associated with chat IDs that consume updates sequentially.
    workers: HashMap<Key, Worker>,
    // The default TX part that consume updates concurrently.
    default_worker: Option<Worker>,

    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,

    state: ShutdownToken,
}

/// An error returned from [`Disatcher::try_dispatch_with_listener`].
pub enum TryDispatchError<R: Requester, L: UpdateListener> {
    /// An error from calling `get_me` while creating dispatcher context.
    GetMe(R::Err),

    /// An error during update listener setup.
    ListenerSetup(L::SetupErr),
}

struct Worker {
    tx: tokio::sync::mpsc::Sender<Update>,
    handle: tokio::task::JoinHandle<()>,
    is_waiting: Arc<AtomicBool>,
}

// TODO: it is allowed to return message as response on telegram request in
// webhooks, so we can allow this too. See more there: https://core.telegram.org/bots/api#making-requests-when-getting-updates

/// A handler that processes updates from Telegram.
pub type UpdateHandler<Err> =
    dptree::Handler<'static, DependencyMap, Result<(), Err>, DpHandlerDescription>;

type DefaultHandler = Arc<dyn Fn(Arc<Update>) -> BoxFuture<'static, ()> + Send + Sync>;

impl<R, Err> Dispatcher<R, Err, DefaultKey>
where
    R: Requester + Clone + Send + Sync + 'static,
    Err: Send + Sync + 'static,
{
    /// Constructs a new [`DispatcherBuilder`] with `bot` and `handler`.
    #[must_use]
    pub fn builder(bot: R, handler: UpdateHandler<Err>) -> DispatcherBuilder<R, Err, DefaultKey>
    where
        Err: Debug,
    {
        const DEFAULT_WORKER_QUEUE_SIZE: usize = 64;

        DispatcherBuilder {
            bot,
            dependencies: DependencyMap::new(),
            handler: Arc::new(handler),
            default_handler: Arc::new(|upd| {
                log::warn!("Unhandled update: {:?}", upd);
                Box::pin(async {})
            }),
            error_handler: LoggingErrorHandler::new(),
            ctrlc_handler: false,
            worker_queue_size: DEFAULT_WORKER_QUEUE_SIZE,
            distribution_f: default_distribution_function,
        }
    }
}

impl<R, Err, Key> Dispatcher<R, Err, Key>
where
    R: Requester + Clone + Send + Sync + 'static,
    Err: Send + Sync + 'static,
    Key: Hash + Eq + Clone,
{
    /// Starts your bot with the default parameters.
    ///
    /// The default parameters are a long polling update listener and log all
    /// errors produced by this listener.
    ///
    /// Each time a handler is invoked, [`Dispatcher`] adds the following
    /// dependencies (in addition to those passed to
    /// [`DispatcherBuilder::dependencies`]):
    ///
    ///  - Your bot passed to [`Dispatcher::builder`];
    ///  - An update from Telegram;
    ///  - [`crate::types::Me`] (can be used in [`HandlerExt::filter_command`]).
    ///
    /// [`HandlerExt::filter_command`]: crate::dispatching::HandlerExt::filter_command
    pub async fn dispatch(&mut self)
    where
        R: Requester + Clone,
        <R as Requester>::GetUpdates: Send,
    {
        let listener = update_listeners::polling_default(self.bot.clone()).await;
        let error_handler =
            LoggingErrorHandler::with_custom_text("An error from the update listener");

        self.dispatch_with_listener(listener, error_handler).await;
    }

    /// Starts your bot with custom `update_listener` and
    /// `update_listener_error_handler`.
    ///
    /// This method adds the same dependencies as [`Dispatcher::dispatch`].
    pub async fn dispatch_with_listener<'a, UListener, Eh>(
        &'a mut self,
        update_listener: UListener,
        update_listener_error_handler: Arc<Eh>,
    ) where
        UListener: UpdateListener + 'a,
        Eh: ErrorHandler<UListener::StreamErr> + 'a,
        UListener::SetupErr: Debug,
    {
        self.try_dispatch_with_listener(update_listener, update_listener_error_handler)
            .await
            .expect("Couldn't prepare dispatching context")
    }

    /// Same as `dispatch_with_listener` but returns a `Err(_)` instead of
    /// panicking when the initial telegram api call (`get_me`) fails.
    ///
    /// Starts your bot with custom `update_listener` and
    /// `update_listener_error_handler`.
    ///
    /// This method adds the same dependencies as [`Dispatcher::dispatch`].
    pub async fn try_dispatch_with_listener<'a, UListener, Eh>(
        &'a mut self,
        mut update_listener: UListener,
        update_listener_error_handler: Arc<Eh>,
    ) -> Result<(), TryDispatchError<R, UListener>>
    where
        UListener: UpdateListener + 'a,
        Eh: ErrorHandler<UListener::StreamErr> + 'a,
    {
        // FIXME: there should be a way to check if dependency is already inserted
        let me = self.bot.get_me().send().await.map_err(TryDispatchError::GetMe)?;
        self.dependencies.insert(me);
        self.dependencies.insert(self.bot.clone());

        let description = self.handler.description();
        let allowed_updates = description.allowed_updates();
        log::debug!("hinting allowed updates: {:?}", allowed_updates);
        update_listener.hint_allowed_updates(&mut allowed_updates.into_iter());

        let mut stop_token = Some(update_listener.stop_token());

        self.state.start_dispatching();

        {
            let stream = update_listener.listen().await.map_err(TryDispatchError::ListenerSetup)?;
            tokio::pin!(stream);

            loop {
                self.remove_inactive_workers_if_needed().await;

                let res = future::select(stream.next(), pin!(self.state.wait_for_changes()))
                    .map(either)
                    .await
                    .map_either(|l| l.0, |r| r.0);

                match res {
                    Either::Left(upd) => match upd {
                        Some(upd) => self.process_update(upd, &update_listener_error_handler).await,
                        None => break,
                    },
                    Either::Right(()) => {
                        if self.state.is_shutting_down() {
                            if let Some(token) = stop_token.take() {
                                log::debug!("Start shutting down dispatching...");
                                token.stop();
                            }
                        }
                    }
                }
            }
        }

        self.workers
            .drain()
            .map(|(_chat_id, worker)| worker.handle)
            .chain(self.default_worker.take().map(|worker| worker.handle))
            .collect::<FuturesUnordered<_>>()
            .for_each(|res| async {
                res.expect("Failed to wait for a worker.");
            })
            .await;

        self.state.done();
        Ok(())
    }

    async fn process_update<LErr, LErrHandler>(
        &mut self,
        update: Result<Update, LErr>,
        err_handler: &Arc<LErrHandler>,
    ) where
        LErrHandler: ErrorHandler<LErr>,
    {
        match update {
            Ok(upd) => {
                if let UpdateKind::Error(err) = upd.kind {
                    log::error!(
                        "Cannot parse an update.\nError: {:?}\n\
                            This is a bug in teloxide-core, please open an issue here: \
                            https://github.com/teloxide/teloxide/issues.",
                        err,
                    );
                    return;
                }

                let worker = match (self.distribution_f)(&upd) {
                    Some(key) => self.workers.entry(key).or_insert_with(|| {
                        let deps = self.dependencies.clone();
                        let handler = Arc::clone(&self.handler);
                        let default_handler = Arc::clone(&self.default_handler);
                        let error_handler = Arc::clone(&self.error_handler);

                        spawn_worker(
                            deps,
                            handler,
                            default_handler,
                            error_handler,
                            Arc::clone(&self.current_number_of_active_workers),
                            Arc::clone(&self.max_number_of_active_workers),
                            self.worker_queue_size,
                        )
                    }),
                    None => self.default_worker.get_or_insert_with(|| {
                        let deps = self.dependencies.clone();
                        let handler = Arc::clone(&self.handler);
                        let default_handler = Arc::clone(&self.default_handler);
                        let error_handler = Arc::clone(&self.error_handler);

                        spawn_default_worker(
                            deps,
                            handler,
                            default_handler,
                            error_handler,
                            self.worker_queue_size,
                        )
                    }),
                };

                worker.tx.send(upd).await.expect("TX is dead");
            }
            Err(err) => err_handler.clone().handle_error(err).await,
        }
    }

    async fn remove_inactive_workers_if_needed(&mut self) {
        let workers = self.workers.len();
        let max = self.max_number_of_active_workers.load(Ordering::Relaxed) as usize;

        if workers <= max {
            return;
        }

        self.remove_inactive_workers().await;
    }

    #[inline(never)] // Cold function.
    async fn remove_inactive_workers(&mut self) {
        let handles = self
            .workers
            .iter()
            .filter(|(_, worker)| {
                worker.tx.capacity() == self.worker_queue_size
                    && worker.is_waiting.load(Ordering::Relaxed)
            })
            .map(|(k, _)| k)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|key| {
                let Worker { tx, handle, .. } = self.workers.remove(&key).unwrap();

                // Close channel, worker should stop almost immediately
                // (it's been supposedly waiting on the channel)
                drop(tx);

                handle
            });

        for handle in handles {
            // We must wait for worker to stop anyway, even though it should stop
            // immediately. This helps in case if we've checked that the worker
            // is waiting in between it received the update and set the flag.
            let _ = handle.await;
        }
    }

    /// Setups the `^C` handler in order to call [`ShutdownToken::shutdown`]
    /// when pressed.
    #[cfg(feature = "ctrlc_handler")]
    #[deprecated(since = "0.10.0", note = "use `enable_ctrlc_handler` on builder instead")]
    pub fn setup_ctrlc_handler(&mut self) -> &mut Self {
        self.setup_ctrlc_handler_inner();
        self
    }

    /// Returns a shutdown token, which can later be used to
    /// [`ShutdownToken::shutdown`].
    pub fn shutdown_token(&self) -> ShutdownToken {
        self.state.clone()
    }
}

impl<R, Err, Key> Dispatcher<R, Err, Key> {
    #[cfg(feature = "ctrlc_handler")]
    fn setup_ctrlc_handler_inner(&mut self) {
        let token = self.state.clone();
        tokio::spawn(async move {
            loop {
                tokio::signal::ctrl_c().await.expect("Failed to listen for ^C");

                match token.shutdown() {
                    Ok(f) => {
                        log::info!("^C received, trying to shutdown the dispatcher...");
                        f.await;
                        log::info!("dispatcher is shutdown...");
                    }
                    Err(_) => {
                        log::info!("^C received, the dispatcher isn't running, ignoring the signal")
                    }
                }
            }
        });
    }
}

impl<R: Requester, L: UpdateListener> Debug for TryDispatchError<R, L>
where
    R: Requester,
    R::Err: Debug,
    L: UpdateListener,
    L::SetupErr: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetMe(e) => f.debug_tuple("GetMe").field(&e).finish(),
            Self::ListenerSetup(e) => f.debug_tuple("ListenerSetup").field(&e).finish(),
        }
    }
}

impl<R, L> fmt::Display for TryDispatchError<R, L>
where
    R: Requester,
    R::Err: Display,
    L: UpdateListener,
    L::SetupErr: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetMe(e) => write!(f, "Error while setting up update listener: {e}"),
            Self::ListenerSetup(e) => write!(f, "Error while setting up update listener: {e}"),
        }
    }
}

impl<R, L> Error for TryDispatchError<R, L>
where
    R: Requester,
    R::Err: Error + Debug + Display + 'static,
    L: UpdateListener,
    L::SetupErr: Error + Debug + Display + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::GetMe(e) => Some(e),
            Self::ListenerSetup(e) => Some(e),
        }
    }
}

fn spawn_worker<Err>(
    deps: DependencyMap,
    handler: Arc<UpdateHandler<Err>>,
    default_handler: DefaultHandler,
    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,
    current_number_of_active_workers: Arc<AtomicU32>,
    max_number_of_active_workers: Arc<AtomicU32>,
    queue_size: usize,
) -> Worker
where
    Err: Send + Sync + 'static,
{
    let (tx, mut rx) = tokio::sync::mpsc::channel(queue_size);
    let is_waiting = Arc::new(AtomicBool::new(true));
    let is_waiting_local = Arc::clone(&is_waiting);

    let deps = Arc::new(deps);

    let handle = tokio::spawn(async move {
        while let Some(update) = rx.recv().await {
            is_waiting_local.store(false, Ordering::Relaxed);
            {
                let current = current_number_of_active_workers.fetch_add(1, Ordering::Relaxed) + 1;
                max_number_of_active_workers.fetch_max(current, Ordering::Relaxed);
            }

            let deps = Arc::clone(&deps);
            let handler = Arc::clone(&handler);
            let default_handler = Arc::clone(&default_handler);
            let error_handler = Arc::clone(&error_handler);

            handle_update(update, deps, handler, default_handler, error_handler).await;

            current_number_of_active_workers.fetch_sub(1, Ordering::Relaxed);
            is_waiting_local.store(true, Ordering::Relaxed);
        }
    });

    Worker { tx, handle, is_waiting }
}

fn spawn_default_worker<Err>(
    deps: DependencyMap,
    handler: Arc<UpdateHandler<Err>>,
    default_handler: DefaultHandler,
    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,
    queue_size: usize,
) -> Worker
where
    Err: Send + Sync + 'static,
{
    let (tx, rx) = tokio::sync::mpsc::channel(queue_size);

    let deps = Arc::new(deps);

    let handle = tokio::spawn(ReceiverStream::new(rx).for_each_concurrent(None, move |update| {
        let deps = Arc::clone(&deps);
        let handler = Arc::clone(&handler);
        let default_handler = Arc::clone(&default_handler);
        let error_handler = Arc::clone(&error_handler);

        handle_update(update, deps, handler, default_handler, error_handler)
    }));

    Worker { tx, handle, is_waiting: Arc::new(AtomicBool::new(true)) }
}

async fn handle_update<Err>(
    update: Update,
    deps: Arc<DependencyMap>,
    handler: Arc<UpdateHandler<Err>>,
    default_handler: DefaultHandler,
    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,
) where
    Err: Send + Sync + 'static,
{
    let mut deps = deps.deref().clone();
    deps.insert(update);

    match handler.dispatch(deps).await {
        ControlFlow::Break(Ok(())) => {}
        ControlFlow::Break(Err(err)) => error_handler.clone().handle_error(err).await,
        ControlFlow::Continue(deps) => {
            let update = deps.get();
            (default_handler)(update).await;
        }
    }
}

fn either<L, R>(x: future::Either<L, R>) -> Either<L, R> {
    match x {
        future::Either::Left(l) => Either::Left(l),
        future::Either::Right(r) => Either::Right(r),
    }
}
#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use teloxide_core::Bot;

    use super::*;

    #[tokio::test]
    async fn test_tokio_spawn() {
        tokio::spawn(async {
            // Just check that this code compiles.
            if false {
                Dispatcher::<_, Infallible, _>::builder(Bot::new(""), dptree::entry())
                    .build()
                    .dispatch()
                    .await;
            }
        })
        .await
        .unwrap();
    }
}
