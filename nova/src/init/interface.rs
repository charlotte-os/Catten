pub enum Error {}

pub trait Interface {
    fn init() -> Result<(), Error>;
}