use crate::parser::{Entries, EntryKind, Value};
use kdl::{KdlEntry, KdlValue};
impl<'a> PartialEq<Entries<'a>> for [KdlEntry] {
    fn eq(&self, other: &Entries<'a>) -> bool {
        let entries = other.entries();
        for entry in entries {
            match entry {
                EntryKind::Argument { position, value } => {
                    let Ok(pos): Result<usize, _> = (*position).try_into() else {
                        return false;
                    };
                    let Some(arg) = self.iter().filter(|v| v.name().is_none()).nth(pos) else {
                        return false;
                    };
                    if value.as_ref().map(|v| arg.value() != v).unwrap_or(false) {
                        return false;
                    };
                }
                EntryKind::Property { name, value } => {
                    let Some(prop) = self
                        .iter()
                        .filter_map(|v| v.name().map(|name| (v, name)))
                        .find(|v| v.1.value() == name)
                        .map(|v| v.0)
                    else {
                        return false;
                    };
                    if value.as_ref().map(|v| prop.value() != v).unwrap_or(false) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl<'a> PartialEq<Value<'a>> for KdlValue {
    fn eq(&self, other: &Value<'a>) -> bool {
        match (self, other) {
            (KdlValue::String(v1), Value::String(v2)) => v1 == v2,
            (KdlValue::Float(v1), Value::FloatingPoing(v2)) => v1 == v2,
            (KdlValue::Integer(v1), Value::Integer(v2)) => v1 == v2,
            (KdlValue::Bool(v1), Value::Boolean(v2)) => v1 == v2,
            (KdlValue::Null, Value::Null) => true,
            _ => false,
        }
    }
}
