use std::sync::{OnceLock, Arc};

use bevy::ecs::{component::Component, removal_detection::RemovedComponents};
use bevy::ecs::query::{Without, With};
use bevy::ecs::system::{SystemId, Query, Commands, ResMut};
use bevy::math::Vec2;
use smallvec::SmallVec;

use crate::dsl::{DslFrom, DslInto};
use crate::events::{EventFlags, CursorAction, CursorFocus, ClickOutside, CursorClickOutside};
use crate::signals::{DataTransfer, Sender, DynamicSender, SignalMapper, SenderBuilder, KeyStorage, Object};
use crate::widgets::drag::DragState;

use self::sealed::EventQuery;

use super::{EvLoseFocus, EvObtainFocus, EvButtonClick, EvTextSubmit, EvTextChange, EvToggleChange, MouseWheel, EvMouseDrag, EvPositionFactor};

/// Event handlers.
#[derive(Debug, Component)]
pub struct Handlers<T: EventHandling> {
    pub context: T::Context,
    pub handlers: SmallVec<[Handler<T>;1]>,
}

impl<T: EventHandling> Default for Handlers<T> {
    fn default() -> Self {
        Self { 
            context: Default::default(), 
            handlers: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Handler<T: EventHandling> {
    OneShotSystem(Arc<OnceLock<SystemId>>),
    Signal(Sender<T::Data>),
    DynamicSignal(DynamicSender),
    GlobalKey(String, SignalMapper),
}

impl<T: EventHandling> DslFrom<Arc<OnceLock<SystemId>>> for Handler<T> {
    fn dfrom(value: Arc<OnceLock<SystemId>>) -> Self {
        Handler::OneShotSystem(value)
    }
}

impl<T: EventHandling> DslFrom<SenderBuilder<T::Data>> for Handler<T> {
    fn dfrom(value: SenderBuilder<T::Data>) -> Self {
        Handler::Signal(value.build())
    }
}

impl<T: EventHandling> DslFrom<Sender<T::Data>> for Handler<T> {
    fn dfrom(value: Sender<T::Data>) -> Self {
        Handler::Signal(value)
    }
}
impl<T: EventHandling> DslFrom<DynamicSender> for Handler<T> {
    fn dfrom(value: DynamicSender) -> Self {
        Handler::DynamicSignal(value)
    }
}

impl<T: EventHandling> DslFrom<String> for Handler<T> {
    fn dfrom(value: String) -> Self {
        Handler::GlobalKey(value, SignalMapper::None)
    }
}

impl<T: EventHandling> DslFrom<&str> for Handler<T> {
    fn dfrom(value: &str) -> Self {
        Handler::GlobalKey(value.to_owned(), SignalMapper::None)
    }
}

impl<T: EventHandling> DslFrom<Arc<OnceLock<SystemId>>> for Handlers<T> {
    fn dfrom(value: Arc<OnceLock<SystemId>>) -> Self {
        Handlers::new(value)
    }
}

impl<T: EventHandling> DslFrom<SenderBuilder<T::Data>> for Handlers<T> {
    fn dfrom(value: SenderBuilder<T::Data>) -> Self {
        Handlers::new(value.build())
    }
}

impl<T: EventHandling> DslFrom<Sender<T::Data>> for Handlers<T> {
    fn dfrom(value: Sender<T::Data>) -> Self {
        Handlers::new(value)
    }
}
impl<T: EventHandling> DslFrom<DynamicSender> for Handlers<T> {
    fn dfrom(value: DynamicSender) -> Self {
        Handlers::new(value)
    }
}


impl<T: EventHandling> Handlers<T> {

    pub fn new_empty() -> Self {
        Self { context: T::new_context(), handlers: SmallVec::new_const() }
    }

    pub fn with(mut self, handler: impl DslInto<Handler<T>>) -> Self {
        self.handlers.push(handler.dinto());
        self
    }

    pub fn new(handler: impl DslInto<Handler<T>>) -> Self {
        Self { context: T::new_context(), handlers: SmallVec::from_const([handler.dinto()]) }
    }

    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }

    pub fn handle(&self, commands: &mut Commands, keys: &mut KeyStorage, data: T::Data) {
        for handler in self.handlers.iter() {
            match handler {
                Handler::OneShotSystem(system) => {
                    if let Some(system) = system.get() {
                        commands.run_system(*system)
                    }
                },
                Handler::Signal(signal) => {
                    signal.send(data.clone());
                },
                Handler::DynamicSignal(signal) => {
                    signal.send(data.clone());
                },
                Handler::GlobalKey(name, mapper) => {
                    keys.set_dyn(name, mapper.map(Object::new(data.clone())));
                },
            }
        }
    }

    pub fn handle_dyn(&self, commands: &mut Commands, keys: &mut KeyStorage, data: Object) {
        for handler in self.handlers.iter() {
            match handler {
                Handler::OneShotSystem(system) => {
                    if let Some(system) = system.get() {
                        commands.run_system(*system)
                    }
                },
                Handler::Signal(signal) => {
                    signal.send_dyn(data.clone())
                },
                Handler::DynamicSignal(signal) => {
                    signal.send_dyn(data.clone());
                },
                Handler::GlobalKey(name, mapper) => {
                    keys.set_dyn(name, mapper.map(data.clone()));
                },
            }
        }
    }
}

/// Trait for a handleable event.
pub trait EventHandling {
    type Data: DataTransfer + Clone;
    type Context: Default + Send + Sync + 'static;
    fn new_context() -> Self::Context;
}

/// Register a `type<T>` that can handle certain events.
pub fn event_handle<T: EventQuery + Send + Sync + 'static> (
    mut commands: Commands,
    mut keys: ResMut<KeyStorage>,
    query: Query<(&T::Component, &Handlers<T>)>,
) {
    for (action, system) in query.iter() {
        if T::validate(&system.context, action) {
            system.handle(&mut commands, &mut keys, T::get_data(&system.context, &action));
        }
    }
}

mod sealed {
    use bevy::ecs::component::Component;
    use super::{EventHandling, CursorAction, EventFlags};

    /// Check for associated event component.
    pub trait EventQuery: EventHandling {
        type Component: Component + Send + Sync;
        fn validate(ctx: &Self::Context, other: &Self::Component) -> bool;
        fn get_data(ctx: &Self::Context, other: &Self::Component) -> Self::Data;
    }

    macro_rules! impl_entity_query_for_mouse_active {
        ($($ident:ident)*) => {
            $(impl EventHandling for $crate::events::$ident {
                type Data = ();
                type Context = ();
                fn new_context() -> Self::Context {
                    ()
                }
            }
            
            impl EventQuery for $crate::events::$ident {
                type Component = CursorAction;
            
                fn validate(_: &Self::Context, other: &Self::Component) -> bool {
                    EventFlags::$ident.contains(other.flags())
                }

                fn get_data(_: &Self::Context, _: &Self::Component) -> () {
                    ()
                }
            })*
        };
    }

    impl_entity_query_for_mouse_active!(
        LeftClick LeftDown DragEnd Drop RightClick
        RightDown MidClick MidDown DoubleClick
    );
}

impl EventHandling for MouseWheel {
    type Data = Vec2;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvMouseDrag {
    type Data = DragState;
    type Context = DragState;
    fn new_context() -> Self::Context {
        DragState::Start
    }
}

impl EventHandling for ClickOutside {
    type Data = ();
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventQuery for ClickOutside {
    type Component = CursorClickOutside;

    fn validate(_: &Self::Context, _: &Self::Component) -> bool {
        true
    }
    fn get_data(_: &Self::Context, _: &Self::Component) -> Self::Data {
        ()
    }
}

macro_rules! impl_entity_query_for_mouse_state {
    ($($ident:ident)*) => {
        $(impl EventHandling for $crate::events::$ident {
            type Data = ();
            type Context = ();
            fn new_context() -> Self::Context {}
        }
        impl EventQuery for $crate::events::$ident {
            type Component = CursorFocus;
        
            fn validate(_: &Self::Context, other: &Self::Component) -> bool {
                EventFlags::$ident.contains(other.flags())
            }
            fn get_data(_: &Self::Context, _: &Self::Component) -> Self::Data {
                ()
            }
        })*
    };
}

impl_entity_query_for_mouse_state! (
    Hover LeftPressed MidPressed RightPressed
    LeftDrag MidDrag RightDrag
);

impl EventHandling for EvLoseFocus {
    type Data = ();
    type Context = ();
    fn new_context() -> Self::Context {}
}


impl EventHandling for EvObtainFocus {
    type Data = ();
    type Context = bool;
    fn new_context() -> Self::Context { false }
}

/// Can never be constructed, hinting at dynamic input, i.e. `Payload`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DynamicData {}

impl EventHandling for EvButtonClick {
    type Data = DynamicData;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvToggleChange {
    type Data = bool;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvTextChange {
    type Data = String;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvTextSubmit {
    type Data = String;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvPositionFactor {
    type Data = f32;
    type Context = ();
    fn new_context() -> Self::Context {}
}

pub fn obtain_focus_detection(
    mut commands: Commands,
    mut keys: ResMut<KeyStorage>,
    mut focused: Query<&mut Handlers<EvObtainFocus>, With<CursorFocus>>,
    mut unfocused: Query<&mut Handlers<EvObtainFocus>, Without<CursorFocus>>,
) {
    for mut handlers in focused.iter_mut() {
        if handlers.context { continue; }
        handlers.context = true;
        handlers.handle(&mut commands, &mut keys, ());
    }
    for mut handlers in unfocused.iter_mut() {
        handlers.context = false;
    }
}

pub fn lose_focus_detection(
    mut commands: Commands,
    mut keys: ResMut<KeyStorage>,
    mut removed: RemovedComponents<CursorFocus>,
    actions: Query<&Handlers<EvLoseFocus>, Without<CursorFocus>>,
) {
    for handlers in actions.iter_many(removed.read()) {
        handlers.handle(&mut commands, &mut keys, ());
    }
}