bitflags! {
    pub struct LogFlags: u8{
        const WRITE_ERROR = 0x01;
        const WRITE_WARNING = 0x02;
        const WRITE_MESSAGE = 0x04;
        const WRITE_TO_CONSOLE = 0x08;
    }
}