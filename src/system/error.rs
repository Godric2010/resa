use crate::system::log::Log;

pub enum ResaResult<T>{
    Ok(T),
    Err()
}

pub enum ResaError{
    InitFailed,
    CallFailed,
    NonCriticalError,
}

impl<T> ResaResult<T> {
    pub fn resolve(self) -> Option<T>{
        match self {
            Self::Ok(t) => Some(t),
            Self::Err() => {
                let msg = format!("Action resolved in an Error!");
                Log::get().write_error(msg.as_str());
                None
            }
        }
    }
}