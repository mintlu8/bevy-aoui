use std::{sync::{Arc, RwLock}, error::Error, marker::PhantomData};
use bevy::ecs::{system::Query, component::Component};
use serde::{Serialize, de::DeserializeOwned};

use crate::dto::Dto;

use self::sealed::SignalCreate;

/// A signal sender
#[derive(Debug, Component)]
pub struct Sender<T=()>(Signal, PhantomData<T>);

// Safety: Save since T is just a marker
unsafe impl<T> Send for Sender<T> {}
// Safety: Save since T is just a marker
unsafe impl<T> Sync for Sender<T> {}

/// A signal receiver
#[derive(Debug, Clone, Component)]
pub struct Receiver<T=()>{ 
    signal: Signal, 
    single: bool,
    p: PhantomData<T>
}


// Safety: Save since T is just a marker
unsafe impl<T> Send for Receiver<T> {}
// Safety: Save since T is just a marker
unsafe impl<T> Sync for Receiver<T> {}


#[derive(Debug, Clone)]
struct Signal(pub(crate) Arc<RwLock<Option<Dto>>>);

impl Signal {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(None)))
    }
    pub fn is_empty(&self) -> bool {
        match self.0.read() {
            Ok(lock) => lock.is_none(),
            Err(_) => true,
        }
    }

    pub fn clean(&self)  {
        let lock = self.0.write();
        if let Ok(mut w) = lock {
            *w = None
        }
    }
}

impl Sender {
    pub fn mark<M>(self) -> Sender<M> {
        Sender(self.0, PhantomData)
    }
}

impl<A> Sender<A> {
    pub fn send<'t, T: Serialize>(&'t self, item: &T) -> Result<(), Box<dyn Error + 't>> {
        let mut lock = self.0.0.write()?;
        match lock.as_mut() {
            Some(dto) => dto.set(item)?,
            None => *lock = Some(Dto::new(item)?),
        };
        Ok(())
    }
}

impl Receiver {
    pub fn mark<M>(self) -> Receiver<M> {
        Receiver {
            signal: self.signal,
            single: self.single,
            p: PhantomData,
        }
    }
}


impl<A> Receiver<A> {
    pub fn poll<T: DeserializeOwned>(&self) -> Option<T> {
        if self.single {
            match self.signal.0.write() {
                Ok(mut lock) => match lock.as_mut().take() {
                    Some(dto) => dto.get().ok(),
                    None => None,
                }
                Err(_) => None,
            }
        } else {
            match self.signal.0.read() {
                Ok(lock) => match lock.as_ref() {
                    Some(dto) => dto.get().ok(),
                    None => None,
                }
                Err(_) => None,
            }
        }
    }
}

mod sealed {
    use std::marker::PhantomData;

    use super::{Sender, Receiver, Signal};

    pub trait SignalCreate {
        fn new() -> Self;
    }

    macro_rules! signal_create {
        ($sender: ident, $first: ident) => {
            impl SignalCreate for ($sender, $first) {
                fn new() -> Self {
                    let signal = Signal::new();
                    (
                        $sender(signal.clone(), PhantomData), 
                        $first{
                            signal,
                            single: true,
                            p: PhantomData
                        }
                    )
                }
            }
        };
        ($sender: ident, $first: ident, $($receivers: ident),*) => {
            impl
                SignalCreate for ($sender, $($receivers),* , $first) {
                fn new() -> Self {
                    let signal = Signal::new();
                    (
                        $sender(signal.clone(), PhantomData),
                        $($receivers{
                            signal: signal.clone(),
                            single: false,
                            p: PhantomData
                        },)*
                        $first{
                            signal,
                            single: false,
                            p: PhantomData
                        }
                    )
                }
            }

            signal_create!($sender, $($receivers),*);
        };
    }

    signal_create!(Sender, 
        Receiver, Receiver, Receiver, Receiver,
        Receiver, Receiver, Receiver, Receiver,
        Receiver, Receiver, Receiver, Receiver
    );   
}

/// Create a spmc signal that can be polled.
/// 
/// ```
/// # /*
/// let (sender, recv_a, recv_b, ...) = signal();
/// # */
/// ```
/// 
/// To have multiple senders or receiver on the same entity,
/// mark them.
/// 
/// ```
/// # /*
/// let sender = sender.mark::<ButtonClick>()
/// # */
/// ```
/// 
/// If registered, this signal is cleared at the end of the frame.
/// 
/// ```
/// # /*
/// app.register_aoui_signal::<ButtonClick>()
/// # */
/// ```
/// 
/// When using a single receiver
/// ```
/// # /*
/// let (sender, receiver) = signal();
/// # */
/// ```
/// 
/// `poll()` will take the data sent,
/// meaning you *might* not need the cleanup routine.
pub fn signal<S: SignalCreate>() -> S {
    S::new()
}

pub fn signal_cleanup<T: 'static>(mut query: Query<&Sender<T>>) {
    query.par_iter_mut().for_each(|x| x.0.clean())
}

