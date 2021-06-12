# eco-engine-rs

A basic game engine written in Rust.

>Ugly, early code.

## Goals:
- [ ] Fix the VSCode launch to be useful.
- [ ] Begin work on basic rendering.
    - [ ] Create a window.
    - [ ] Create a Vulkan instance.
    - [ ] Enumerate Vulkan's devices.
    - [ ] Create Vulkan Devices.
    - [ ] Create render queues.
        - [ ] CPU-GPU Accessible buffers.
        - [ ] Command buffers.
    - [ ] Create the compute pipelines.
    - [ ] Handle shaders.
        - [ ] Parse and compile shaders on launch/compile (with custom build script).
- [ ] Begin work on game engine-related stuff.
- [ ] Improve module imports && documentation.
- [ ] Major refactoring, code shortening, and bugfixing.
- [ ] Implement a scripting language.

--------------------------------------------------
## Formatting Rules:

- `rust-analyzer`'s format is what I use.