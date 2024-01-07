use nvim_oxi::{self as oxi, lua, Object};
use oxi::conversion::{self, FromObject, ToObject};
use oxi::serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Opts {
}

impl FromObject for Opts {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl ToObject for Opts {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl lua::Poppable for Opts {
    unsafe fn pop(
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_object(obj)
            .map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl lua::Pushable for Opts {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<std::ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}
