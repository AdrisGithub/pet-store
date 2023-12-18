use wjp::{map, ParseError, Serialize, SerializeHelper, Values};

#[derive(Debug)]
pub struct Test {
    message: String,
}

impl TryFrom<Values> for Test {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let struc = value.get_struct().ok_or(ParseError::new())?;
        let message = struc.get_val_opt("message", |val| val.get_string()).ok_or(ParseError::new())?;
        Ok(Self {
            message
        })
    }
}

impl Serialize for Test {
    fn serialize(&self) -> Values {
        Values::Struct(map!(("message",self.message.serialize())))
    }
}

impl From<String> for Test {
    fn from(value: String) -> Self {
        Self {
            message: value
        }
    }
}