use std::hash::{Hash, Hasher};

pub const ROOT_SCALE: i8 = 35;
pub const MIN_SCALE: i8 = -35;
pub const SCALE_COUNT: usize = (ROOT_SCALE - MIN_SCALE + 1) as usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalCell(i8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalCell3 {
    x: LocalCell,
    y: LocalCell,
    z: LocalCell,
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkAddress {
    len: u8,
    cells: [LocalCell3; SCALE_COUNT],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkHandle(u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk<T> {
    handle: ChunkHandle,
    address: ChunkAddress,
    contents: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChunkAddressError {
    LocalCellOutOfRange(i8),
    EmptyPath,
    ChildPastMinScale,
}

impl LocalCell {
    pub const MIN: i8 = -5;
    pub const MAX: i8 = 4;

    pub fn try_new(value: i8) -> Result<Self, ChunkAddressError> {
        if value < Self::MIN || value > Self::MAX {
            Err(ChunkAddressError::LocalCellOutOfRange(value))
        } else {
            Ok(Self(value))
        }
    }

    pub const fn get(self) -> i8 {
        self.0
    }
}

impl LocalCell3 {
    pub const ZERO: Self = Self {
        x: LocalCell(0),
        y: LocalCell(0),
        z: LocalCell(0),
    };

    pub const fn new(x: LocalCell, y: LocalCell, z: LocalCell) -> Self {
        Self { x, y, z }
    }

    pub fn try_new(x: i8, y: i8, z: i8) -> Result<Self, ChunkAddressError> {
        Ok(Self::new(LocalCell::try_new(x)?, LocalCell::try_new(y)?, LocalCell::try_new(z)?))
    }

    pub const fn into_array(self) -> [i8; 3] {
        [self.x.get(), self.y.get(), self.z.get()]
    }
}

impl ChunkAddress {
    pub const ROOT_SCALE: i8 = ROOT_SCALE;
    pub const MIN_SCALE: i8 = MIN_SCALE;
    pub const SCALE_COUNT: usize = SCALE_COUNT;

    pub fn root(cell: LocalCell3) -> Self {
        let mut cells = [LocalCell3::ZERO; Self::SCALE_COUNT];
        cells[0] = cell;
        Self { len: 1, cells }
    }

    pub fn origin() -> Self {
        Self::root(LocalCell3::ZERO)
    }

    pub fn from_cells(path: &[LocalCell3]) -> Result<Self, ChunkAddressError> {
        if path.is_empty() {
            return Err(ChunkAddressError::EmptyPath);
        }
        if path.len() > Self::SCALE_COUNT {
            return Err(ChunkAddressError::ChildPastMinScale);
        }

        let mut cells = [LocalCell3::ZERO; Self::SCALE_COUNT];
        cells[..path.len()].copy_from_slice(path);
        Ok(Self {
            len: path.len() as u8,
            cells,
        })
    }

    pub fn scale(&self) -> i8 {
        Self::ROOT_SCALE - (self.len as i8 - 1)
    }

    pub fn depth(&self) -> usize {
        self.len as usize
    }

    pub fn cells(&self) -> &[LocalCell3] {
        &self.cells[..self.depth()]
    }

    pub fn leaf_cell(&self) -> LocalCell3 {
        self.cells[self.depth() - 1]
    }

    pub fn parent(&self) -> Option<Self> {
        if self.len <= 1 {
            return None;
        }

        let mut out = *self;
        out.len -= 1;
        out.cells[out.len as usize] = LocalCell3::ZERO;
        Some(out)
    }

    pub fn child(&self, cell: LocalCell3) -> Result<Self, ChunkAddressError> {
        if self.depth() == Self::SCALE_COUNT {
            return Err(ChunkAddressError::ChildPastMinScale);
        }

        let mut out = *self;
        out.cells[out.len as usize] = cell;
        out.len += 1;
        Ok(out)
    }

    pub fn is_ancestor_of(&self, other: &Self) -> bool {
        self.depth() < other.depth() && other.cells().starts_with(self.cells())
    }
}

impl PartialEq for ChunkAddress {
    fn eq(&self, other: &Self) -> bool {
        self.cells() == other.cells()
    }
}

impl Eq for ChunkAddress {}

impl Hash for ChunkAddress {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cells().hash(state);
    }
}

impl ChunkHandle {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl<T> Chunk<T> {
    pub fn new(handle: ChunkHandle, address: ChunkAddress, contents: T) -> Self {
        Self { handle, address, contents }
    }

    pub fn handle(&self) -> ChunkHandle {
        self.handle
    }

    pub fn address(&self) -> &ChunkAddress {
        &self.address
    }

    pub fn contents(&self) -> &T {
        &self.contents
    }

    pub fn into_contents(self) -> T {
        self.contents
    }
}
