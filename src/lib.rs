use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use mgpu::wasm_bindgen::prelude::*;

fn _run() -> Result<(), mgpu::winit::error::EventLoopError> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut app = mgpu::util::App::new("MPS", Rc::new(mps_system::system::System::new()));
    let event_loop = mgpu::winit::event_loop::EventLoop::builder().build()?;
    event_loop.set_control_flow(mgpu::winit::event_loop::ControlFlow::Poll);

    event_loop.run_app(&mut app)?;
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run() {
    _run().unwrap();
}