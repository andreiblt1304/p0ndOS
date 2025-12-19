# p0nd_os

An x86_64 hobby kernel that stays close to the metal: `#![no_std]`, `#![no_main]`, a hand-built memory map, and interrupt handling without an OS or libc underneath. It is intentionally small to show low-level Rust and bare-metal techniques.

## Key capabilities
- Custom target spec (`x86_64-p0nd_os.json`) with panic-abort, no SIMD, and red-zone disabled so interrupts can safely use the stack.
- VGA text console and serial output so panics and tests are visible even before higher-level drivers exist (`src/vga_buffer.rs`, `src/serial.rs`).
- GDT + TSS setup with a dedicated double-fault IST stack, plus an IDT that wires timer, keyboard, breakpoint, and page-fault handlers (`src/gdt.rs`, `src/interrupts.rs`).
- Virtual memory primitives: page-table init from the bootloader offset, a `BootInfoFrameAllocator` that iterates the firmware memory map, and helpers to map physical frames (`src/memory.rs`).
- Heap carved out of a manually mapped virtual range with a fixed-size-block allocator by default (`src/allocator.rs`, `src/allocator/fixed_size_block.rs`), plus alternate bump and linked-list allocators for comparison.
- Custom test harness that runs inside QEMU, reports via the serial port, and exits with ISA debug port codes so automated tests can assert success (`src/lib.rs`, `tests/*`).
- Tooling is configured in `.cargo/config.toml` to target this kernel by default, build `core`/`alloc` for the custom target, and route `cargo test` through `bootimage runner`.
- Minimal async/task system with a waker-aware executor built on `crossbeam_queue` and futures-based keyboard input streams (`src/task/*`).

## Boot flow
1. The bootloader jumps to the kernel entry declared with `entry_point!` in `src/main.rs`.
2. `kernel_main` calls `p0nd_os::init()` to install the GDT/TSS, load the IDT, remap the PICs, and enable interrupts.
3. Virtual memory is brought up using the physical memory offset supplied by the bootloader; a frame allocator is built from the BIOS/UEFI memory map.
4. `allocator::init_heap` maps a contiguous virtual heap and installs the global allocator, enabling `Box`, `Vec`, and `Rc` usage in a `no_std` context.
5. Async tasks are spawned on the executor (e.g., a demo `example_task` and keyboard printer), and the executor runs forever, halting the CPU when idle.

## Module guide
- `src/main.rs`: Kernel entry point, demo allocations, and panic handlers for test vs. normal boots.
- `src/lib.rs`: Common init routine, QEMU exit helpers, custom test harness plumbing, and an `hlt` loop.
- `src/vga_buffer.rs`: Minimal text-mode console built on volatile memory writes; provides `print!/println!` macros that are interrupt-safe via spinlocks.
- `src/serial.rs`: 16550 UART driver with `serial_print!` macros used for headless testing.
- `src/gdt.rs`: Builds the GDT and TSS, installs selectors, and preallocates an IST stack for double-fault recovery.
- `src/interrupts.rs`: IDT setup, page-fault logging, timer and keyboard IRQ handlers, and PIC end-of-interrupt signaling.
- `src/memory.rs`: Page-table initialization from the active level-4 table, bootloader-backed frame allocator, and an example mapping helper.
- `src/allocator.rs`: Heap mapping and the global fixed-size-block allocator; includes a `Dummy` allocator and alternate bump (`allocator/bump.rs`) and linked-list (`allocator/linked_list.rs`) allocators.
- `src/task/executor.rs`: Waker-based task executor backed by a bounded queue that sleeps the CPU when idle.
- `src/task/simple_executor.rs`: A minimal executor example with a dummy waker, useful for understanding the scheduling basics.
- `src/task/task_struct.rs`: `Task` wrapper that boxes futures and assigns stable task IDs.
- `src/task/keyboard.rs`: Scancode queue and async stream that decodes keystrokes to drive `print_keypresses()`.
- `tests/*`: Bootable integration tests for printing, heap allocation, stack overflows with custom IST, and panic behavior; all exit QEMU via port `0xf4`.

## Building and running
Prereqs: nightly Rust, `rustup component add llvm-tools-preview`, `rustup target add x86_64-unknown-none`, and `cargo install bootimage`. QEMU is required to run the built image.

Build a bootable image:
```bash
cargo bootimage --target x86_64-p0nd_os.json
```

Run it in QEMU (mirrors the `bootimage` test args):
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-p0nd_os/debug/bootimage-p0nd_os.bin \
  -serial stdio -display none -device isa-debug-exit,iobase=0xf4,iosize=0x04
```

Run the QEMU-backed test suite:
```bash
cargo test --target x86_64-p0nd_os.json
```

## Notes on low-level choices
- Interrupt safety: output routines mask interrupts while holding spinlocks to avoid deadlocks on nested interrupts.
- Paging without std: all virtual memory work is done with `OffsetPageTable` and raw pointer math; the target disables the red zone so interrupt frames do not clobber stack data.
- Heap in a freestanding environment: the heap is manually mapped before the global allocator is initialized, demonstrating controlled memory management without a host OS.
- Firmware-driven memory discovery: the `BootInfoFrameAllocator` consumes the firmware-supplied memory map and yields only usable 4 KiB frames.

## Extending the kernel
- Add new device drivers by wiring handlers in `src/interrupts.rs` and acknowledging PICs via `notify_end_of_interrupt`.
- Map new regions by creating `Page`/`PhysFrame` pairs and using `memory::create_example_mapping` as a template.
- Swap allocators by replacing the global `FixedSizeBlockAllocator` in `src/allocator.rs` with the bump or linked-list allocator variants.

## What this demonstrates
This codebase is a compact but complete example of bringing up Rust in a `no_std` environment: custom targets, hand-rolled interrupt and descriptor tables, direct port I/O, a bespoke heap, and QEMU-driven tests that run inside the kernel. It is designed to showcase systems-level Rust skills, not to be a full OS.
