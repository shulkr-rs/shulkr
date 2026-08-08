#[derive(Debug, Clone)]
pub(crate) struct DragState {
    pub action: DragAction,
    pub slots: Vec<i16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DragAction {
    Left,
    Right,
    Middle,
}
