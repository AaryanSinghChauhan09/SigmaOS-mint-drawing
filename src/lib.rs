// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mint-drawing: mint drawing component model.
//!
//! Real implementation: drawings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drawing {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintDrawingError {
    EmptyDrawingName,
    DuplicateDrawing(String),
    EmptyEventSubject,
}

impl fmt::Display for MintDrawingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDrawingName => write!(f, "drawing name must not be empty"),
            Self::DuplicateDrawing(id) => write!(f, "duplicate drawing {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintDrawingError {}

pub struct MintDrawing {
    pub name: String,
    drawings: BTreeMap<String, Drawing>,
    events: BTreeMap<String, Event>,
}

impl MintDrawing {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            drawings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_drawing(&mut self, item: Drawing) -> Result<(), MintDrawingError> {
        if item.name.trim().is_empty() {
            return Err(MintDrawingError::EmptyDrawingName);
        }
        let id = item.id.clone();
        if self.drawings.contains_key(&id) {
            return Err(MintDrawingError::DuplicateDrawing(id.clone()));
        }
        self.drawings.insert(id, item);
        Ok(())
    }

    pub fn drawing_count(&self) -> usize {
        self.drawings.len()
    }

    pub fn get_drawing(&self, id: &str) -> Option<&Drawing> {
        self.drawings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintDrawingError> {
        if event.subject.trim().is_empty() {
            return Err(MintDrawingError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_drawing(id: &str, name: &str) -> Drawing {
        Drawing {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintDrawing::new("MintDrawing");
        assert_eq!(m.name, "MintDrawing");
        assert_eq!(m.drawing_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_drawing() {
        let mut m = MintDrawing::new("MintDrawing");
        m.add_drawing(sample_drawing("a1", "Alpha")).unwrap();
        assert_eq!(m.drawing_count(), 1);
        assert!(m.get_drawing("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintDrawing::new("MintDrawing");
        assert!(m.add_drawing(sample_drawing("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintDrawing::new("MintDrawing");
        m.add_drawing(sample_drawing("x", "One")).unwrap();
        assert!(m.add_drawing(sample_drawing("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintDrawing::new("MintDrawing");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
