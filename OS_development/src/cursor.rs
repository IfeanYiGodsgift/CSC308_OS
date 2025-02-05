#![allow(dead_code)]
use x86_64::instructions::port::Port;

///These functions are for handling cursor in VGA Text Mode.
/// VGA text mode was introduced in 1987 by IBM as part of the VGA standard for its IBM PS/2 computers
/// See https://en.wikipedia.org/wiki/VGA_text_mode
/// See C and Assembly equivalents at https://wiki.osdev.org/Text_Mode_Cursor#Without_the_BIOS
/// Here, we are leveraging on Rust x86_64::instructions that wrap the respective assembly language codes
/// that enable us interact directly with the I/O mapped hardware. Beware!.
pub fn disable_cursor() {
    let mut port = Port::new(0x3D4);//0x3D4 is I/O port for text cursor
    unsafe {
        port.write(0x0Au8); // Set cursor start register
    }
    let mut port = Port::new(0x3D5); //0x3D5 is like the data entry field
    unsafe {
        port.write(0x20u8); // Disable cursor
    }
}

pub fn enable_cursor() {
    let mut port = Port::new(0x3D4);
    unsafe {
        port.write(0x0Au8); // Set cursor start register
    }
    let mut port = Port::new(0x3D5);
    unsafe {
        port.write(0xA0u8); // Enable cursor with a blinking block
    }
}

pub fn get_cursor_pos() -> (u16, u16) {
    let mut pos: u16;
    let mut port = Port::new(0x3D4);
    unsafe {
        port.write(0x0Fu8);
    }
    port = Port::new(0x3D5);
    unsafe {
        pos = port.read() as u16;
    }
    port = Port::new(0x3D4);
    unsafe {
        port.write(0x0Eu8);
    }
    port = Port::new(0x3D5);
    unsafe {
        pos |= (port.read() as u16) << 8;
    }
    let x = pos % 80;
    let y = pos / 80;
    (x, y)
}

pub fn update_cursor(x: u16, y: u16) {
    let pos = (y as u16) * 80 + (x as u16);
    let mut port = Port::new(0x3D4);
    unsafe {
        port.write(0x0Fu8);
    }
    port = Port::new(0x3D5);
    unsafe {
        port.write((pos & 0xFF) as u8);
    }
    port = Port::new(0x3D4);
    unsafe {
        port.write(0x0Eu8);
    }
    port = Port::new(0x3D5);
    unsafe {
        port.write(((pos >> 8) & 0xFF) as u8);
    }
}