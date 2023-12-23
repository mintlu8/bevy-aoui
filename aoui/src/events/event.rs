use bevy::ecs::component::Component;

/// Represents hovering, clicking or dragging.
#[derive(Debug, Component, Clone, Copy)]
#[component(storage="SparseSet")]
pub struct CursorFocus(pub(super) EventFlags);

impl CursorFocus {
    pub fn flags(&self) -> EventFlags {
        self.0
    }
    pub fn is(&self, flag: EventFlags) -> bool {
        self.0 == flag
    }
    pub fn intersects(&self, flag: EventFlags) -> bool {
        self.0.0 & flag.0 > 0
    }
}

/// Represents a cursor event like `OnMouseDown`.
#[derive(Debug, Component, Clone, Copy)]
#[component(storage="SparseSet")]
pub struct CursorAction(pub(super) EventFlags);

impl CursorAction {
    pub fn flags(&self) -> EventFlags {
        self.0
    }
    pub fn is(&self, flag: EventFlags) -> bool {
        self.0 == flag
    }
    pub fn intersects(&self, flag: EventFlags) -> bool {
        self.0.0 & flag.0 > 0
    }
}


/// Represents cursor clicking outside the sprite's hitbox.
#[derive(Debug, Component)]
#[component(storage="SparseSet")]
pub struct CursorClickOutside;

tlbf::tlbf!(
    /// Flags for cursor events.
    ///
    /// Valid listeners are `Hover`, `*Click`, `*Drag`, `DoubleClick`, `Drop` and `ClickOutside`.
    ///
    /// * `Hover` listens for `Hover`,
    /// * `Click` listens for `Down`, `Up` and `Pressed`
    /// * `Drag` listens for `Down`, `DragEnd` and `Drag`
    /// * `DoubleClick` listens for `DoubleClick`, which replaces `Click` or `DragEnd`
    /// * `Drop` listens for `Drop`
    /// * `ClickOutside` listens for mouse up outside
    ///
    /// Events are emitted as 3 separate components, each frame a sprite can receive at most one of each:
    /// * `CursorFocus`: `Hover`, `Pressed`, `Drag`.
    /// * `CursorAction`: `Down`, `Click`, `DragEnd`, `DoubleClick`, `Drop`.
    /// * `CursorClickOutside`: `ClickOutside`.
    ///
    /// Details:
    /// * `Click` requires mouse up and mouse down be both inside a sprite.
    /// * `ClickOutside` requires mouse up be outside of a sprite and the sprite not being dragged.
    /// * Dragged sprite will receive `Down` from other mouse buttons regardless of their handlers.
    /// * There is in fact no `MouseUp`.
    #[derive(Component)]
    pub EventFlags: u32 {
        Idle,
        Hover,
        LeftDrag,
        LeftDown,
        LeftPressed,
        LeftClick,
        DoubleClick,
        MidDown,
        MidPressed,
        MidClick,
        MidDrag,
        RightDown,
        RightPressed,
        RightClick,
        RightDrag,
        Drop,
        DragEnd,
        ClickOutside,
        MouseWheel,
    }
);

type EvIdle = Idle;
type EvHover = Hover;
type EvLeftDrag = LeftDrag;
type EvLeftDown = LeftDown;
type EvLeftPressed = LeftPressed;
type EvLeftClick = LeftClick;
type EvDoubleClick = DoubleClick;
type EvMidDown = MidDown;
type EvMidPressed = MidPressed;
type EvMidClick = MidClick;
type EvMidDrag = MidDrag;
type EvRightDown = RightDown;
type EvRightPressed = RightPressed;
type EvRightClick = RightClick;
type EvRightDrag = RightDrag;
type EvDrop = Drop;
type EvDragEnd = DragEnd;
type EvClickOutside = ClickOutside;
type EvMouseWheel = MouseWheel;


impl EventFlags {
    pub const fn const_or(self, other: EventFlags) -> Self{
        Self(self.0 | other.0)
    }
}

/// An event sent if widget is being dragged.
/// 
/// Sends the state `Start`, `Continue`, `End`.
/// Delta can be found in the input context.
#[derive(Debug)]
pub enum EvMouseDrag{}

/// An event sent if widget has lost focus (drag, hover, pressed).
#[derive(Debug)]
pub enum EvLoseFocus{}

/// An event sent if widget has obtained focus (drag, hover, pressed).
#[derive(Debug)]
pub enum EvObtainFocus{}

/// An event that sends the `Payload` value of a widget in the button family.
#[derive(Debug)]
pub enum EvButtonClick{}

/// An event that sends the `bool` value of a Toggle/CheckButton.
#[derive(Debug)]
pub enum EvToggleChange{}

/// An event that sends the content of a text input
/// on change.
#[derive(Debug)]
pub enum EvTextChange{}

/// An event that sends the content of a text input via
/// pressing enter or signal piping.
#[derive(Debug)]
pub enum EvTextSubmit{}

/// An event that sends a floating point number based on 
/// the position of a sprite, via either scrolling or dragging.
#[derive(Debug)]
pub enum EvPositionFactor{}