use ptx_builder::error::Result;
use ptx_builder::prelude::*;

fn main() -> Result<()> {
    std::env::set_var("CARGO_BUILD_PIPELINING", "false");
    let builder = Builder::new("cuda_core")?;
    CargoAdapter::with_env_var("KERNEL_PTX_PATH").build(builder);
}
