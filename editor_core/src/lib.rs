use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientEvent {
    KeyPress(String),
    InitialRequest,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub enum ServerUpdate {
    CursorMoved {
        buffer_id: usize,
        new_pos: BufferPosition,
    },
    BufferChanged {
        buffer_id: usize,
        new_pos: Option<BufferPosition>,
    },
    CloseBuffer {
        buffer_id: usize,
    },
    BufferNew(Buffer),
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash)]
pub struct Buffer {
    pub contents: Vec<String>,
    pub path: PathBuf,
    id: usize,
    pub position: BufferPosition,
}

impl Buffer {
    pub fn new(path: &Path, id: usize) -> std::io::Result<Self> {
        let path = path.to_owned();

        let contents: Vec<_> = std::fs::read_to_string(&path)?
            .lines()
            .map(ToOwned::to_owned)
            .collect();

        Ok(Self {
            contents,
            path,
            id,
            position: BufferPosition::default(),
        })
    }

    pub fn from_str(contents: &str, id: usize) -> Self {
        let path = PathBuf::new();
        let contents: Vec<_> = contents.lines().map(ToOwned::to_owned).collect();

        Self {
            contents,
            path,
            id,
            position: BufferPosition::default(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Hash)]
pub struct BufferPosition {
    pub line: usize,
    pub column_real: usize,
}
