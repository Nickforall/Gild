//! This module contains all of the built-in validators

mod size;
mod strings;
mod email;

pub use self::size::MaxSize;
pub use self::size::MinSize;
pub use self::strings::Contain;
pub use self::strings::NotContain;
pub use self::strings::Empty;
pub use self::email::IsEmail;
