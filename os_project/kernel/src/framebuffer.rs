use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    White = 15,
}

#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct FrameBufferWriter {
    buffer: &'static mut Buffer,
    cursor_x: usize,
    cursor_y: usize,
}

impl FrameBufferWriter {
    pub fn write_byte(&mut self, byte: u8) {
        if self.cursor_x >= BUFFER_WIDTH {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }
        if self.cursor_y >= BUFFER_HEIGHT {
            self.scroll();
            self.cursor_y = BUFFER_HEIGHT - 1;
        }
        let row = self.cursor_y;
        let col = self.cursor_x;
        self.buffer.chars[row][col].write(ScreenChar {
            ascii_character: byte,
            color_code: Color::White as u8,
        });
        self.cursor_x += 1;
    }

    fn scroll(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: b' ',
                color_code: Color::Black as u8,
            });
        }
    }
}