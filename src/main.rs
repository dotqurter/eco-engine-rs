mod data;
mod vulkan;

fn main() {
    std::thread::spawn(|| println!("Test! This should call."));

    let w = match vulkan::init::Graphics::new() {
        Ok(mut app) => std::thread::spawn(move || app.render_loop()),
        Err(what)   => panic!("Failed to create Graphics struct. {}", what)
    };

    println!("bruh");

    //works as intended: need to implement the main window loop.
    w.join();
}
