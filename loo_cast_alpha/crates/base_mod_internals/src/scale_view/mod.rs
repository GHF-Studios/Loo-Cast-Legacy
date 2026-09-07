use crate::chunk::ChunkAddress;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScaleView {
    focus: ChunkAddress,
    downward_depth: u8,
}

impl ScaleView {
    pub fn new(focus: ChunkAddress) -> Self {
        Self { focus, downward_depth: 1 }
    }

    pub fn with_downward_depth(mut self, downward_depth: u8) -> Self {
        self.downward_depth = downward_depth;
        self
    }

    pub fn focus(&self) -> ChunkAddress {
        self.focus
    }

    pub fn downward_depth(&self) -> u8 {
        self.downward_depth
    }

    pub fn set_focus(&mut self, focus: ChunkAddress) {
        self.focus = focus;
    }
}
