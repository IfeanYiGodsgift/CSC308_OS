#![no_std]
#![no_main]

use bootloader_api::config::Mapping;
use core::fmt::Write;
use x86_64::instructions::hlt;

// Use the entry_point macro to register the entry point function:
bootloader_api::entry_point!(my_entry_point);

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

static mut FRAMEBUFFER: [[u8; 80]; 25] = [[b' '; 80]; 25];

fn my_entry_point(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    // Using a raw pointer instead of mutable reference to static
    let mut writer = unsafe { FrameBufferWriter::new(&raw mut FRAMEBUFFER) };

    writeln!(writer, "Hello, world!").unwrap();
    writeln!(writer, "Test").unwrap();

    loop {
        hlt(); // Stop x86_64 from being unnecessarily busy while looping
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Print panic message to framebuffer if possible
    let mut writer = unsafe { FrameBufferWriter::new(&raw mut FRAMEBUFFER) };
    writeln!(writer, "Kernel Panic: {}", info).ok(); // Ignore potential errors here

    loop {
        hlt();
    }
}

pub struct FrameBufferWriter {
    width: usize,
    height: usize,
    cursor_x: usize,
    cursor_y: usize,
    buffer: &'static mut [[u8; 80]; 25],
}

impl FrameBufferWriter {
    pub fn new(buffer: &'static mut [[u8; 80]; 25]) -> Self {
        Self {
            width: 80,
            height: 25,
            cursor_x: 0,
            cursor_y: 0,
            buffer,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
        } else {
            if self.cursor_x >= self.width {
                self.new_line();
            }
            self.buffer[self.cursor_y][self.cursor_x] = byte;
            self.cursor_x += 1;
        }
    }

    fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        if self.cursor_y >= self.height {
            self.scroll();
            self.cursor_y = self.height - 1;
        }
    }

    fn scroll(&mut self) {
        for row in 1..self.height {
            self.buffer[row - 1] = self.buffer[row];
        }
        self.clear_row(self.height - 1);
    }

    fn clear_row(&mut self, row: usize) {
        self.buffer[row].fill(b' ');
    }
}

impl core::fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}