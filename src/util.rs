use salah::prelude::*;

pub fn to_local(time: salah::DateTime<salah::Utc>) -> salah::DateTime<salah::Local> {
    DateTime::<Local>::from(time)
}
