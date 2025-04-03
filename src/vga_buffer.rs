#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // 背景色与前景色组合
        // (background as u8) << 4：将背景色的值左移 4 位，使其占据高 4 位。
        // (foreground as u8)：前景色的值保持不变，占据低 4 位。
        // | 操作符：将背景色和前景色的值按位或运算，组合成一个完整的 u8 值。
        // 例如：
        // ColorCode::new(Color::LightRed, Color::DarkGray);
        // foreground: 12, background: 8
        // (8 as u8) << 4 = 128（二进制：10000000）
        // (12 as u8) = 12（二进制：00001100）
        // 按位或运算：128 | 12 = 140（二进制：10001100）
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
    // chars: [[ScreenChar; 80]; 25],
}

pub struct Writer {
    column_postion: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // 换行
            b'\n' => self.new_line(),
            byte => {
                if (self.column_postion >= BUFFER_WIDTH) {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_postion;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };

                self.column_postion += 1;
            }
        }
    }

    fn new_line(&mut self) {
        /* TODO */
    }

    pub fn write_string(&mut self, s: &str) {
        // 遍历字符串的每个字节
        for byte in s.bytes() {
            match byte {
                // 可以是能打印的 ASCII 码字节，也可以是换行符
                // 匹配到ASCII 码字节 或 \n换行
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 无法识别的字节，则默认输出0xfe
                _ => self.write_byte(0xfe),
            }
        }
    }
}

pub fn print_something() {
    let mut writer = Writer {
        column_postion: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
