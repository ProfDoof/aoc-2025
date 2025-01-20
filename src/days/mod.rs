pub (crate) mod one;

pub(crate) trait Day {
    fn run(&self, inputs: std::path::PathBuf) -> anyhow::Result<()>;
}