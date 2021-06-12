# eco-engine-rs

A basic game engine written in Rust.

>Ugly, early code.

## Goals:
- [ ] Begin work on basic rendering.
    - [x] Create a Vulkan instance.
    - [x] Enumerate Vulkan's Physical Devices.
    - [x] Create Vulkan Devices.
    - [ ] Create render queues.
        - [ ] CPU-GPU Accessible buffers.
        - [ ] Command buffers.
    - [ ] Create the compute pipelines.
    - [ ] Handle shaders.
        - [ ] Parse and compile shaders on launch/compile (with custom build script).
    - [ ] Create a window.
- [ ] Begin work on game engine-related stuff.
- [ ] Improve module imports && documentation.
- [ ] Major refactoring, code shortening, and bugfixing.
- [ ] Implement a scripting language.

--------------------------------------------------
## Formatting Rules:

- `rust-analyzer`'s format is what I use.