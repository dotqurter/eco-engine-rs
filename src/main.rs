mod vulkan;
mod data;

fn main() {
    vulkan::init::init(crate::data::application::Application::default());
}