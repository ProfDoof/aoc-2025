pub (crate) mod one;
pub (crate) mod two;
pub (crate) mod three;

pub(crate) trait Day {
    fn run(&self, inputs: std::path::PathBuf) -> anyhow::Result<()>;
}