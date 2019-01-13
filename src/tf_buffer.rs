
pub mod tf {

use rosrust::Time;

    pub enum InvalidFrameIdDescription {
        InvalidCharacters,
        InvalidPrefix,
    }

    pub struct FrameId {
        pub name: String
    }

    impl FrameId {
        pub fn new<S: Into<String>>(name: S) -> Result<FrameId, InvalidFrameIdDescription> {
            Ok(FrameId{name: name.into()})
        }
    }

    struct Transform {
        from: FrameId,
        to: FrameId,
        stamp: Time,

    }

    struct Buffer {

    }

}
