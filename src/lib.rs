#![allow(non_camel_case_types)]

use libc::{c_int, c_float, c_void};
use std::ffi::CString;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[repr(C)]
struct CColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[repr(C)]
struct CVector2 {
    x: f32,
    y: f32,
}

/*================================
       Font stuff, internals.
  ================================*/

#[repr(C)]
#[derive(Clone)]
struct CTexture {
    id: u32,
    width: i32,
    height: i32,
    mipmaps: i32,
    format: i32,
}

type CTexture2D = CTexture;

#[repr(C)]
struct CRectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[repr(C)]
struct CImage {
    data: c_void,
    width: i32,
    height: i32,
    mipmaps: i32,
    format: i32,
}

#[repr(C)]
struct CGlyphInfo {
    value: i32,
    offset_x: i32,
    offset_y: i32,
    advance_x: i32,
    image: CImage,
}

#[repr(C)]
#[derive(Clone)]
struct CFont {
    base_size: i32,
    glyph_count: i32,
    glyph_padding: i32,
    texture: CTexture2D,
    recs: *mut CRectangle,
    glyphs: *mut CGlyphInfo,
}

#[link(name = "raylib", kind = "static")]
extern {
    fn InitWindow(width: c_int, height: c_int, title: *const i8);
    fn CloseWindow();
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: CColor);
    fn DrawText(text: *const i8, pos_x: c_int, pos_y: c_int, font_size: c_int, color: CColor);
    fn DrawRectangle(pos_x: c_int, pos_y: c_int, width: c_int, height: c_int, color: CColor);
    //fn DrawRectangle(position: CVector2, size: CVector2, color: CColor);
    fn DrawCircle(center_x: c_int, center_y: c_int, radius: c_float, color: CColor);
    fn IsKeyDown(key: c_int) -> bool;
    fn GetFrameTime() -> c_float;
    fn GetMousePosition() -> CVector2;
    fn GetScreenWidth() -> c_int;
    fn GetScreenHeight() -> c_int;
    fn GetFPS() -> c_int;
    fn DrawLineEx(start_pos: CVector2, end_pos: CVector2, thickness: c_float, color: CColor);
    fn IsMouseButtonPressed(button: c_int) -> bool;
    fn GetFontDefault() -> CFont;
    fn SetTextLineSpacing(spacing: c_int);
    fn MeasureTextEx(font: CFont, text: *const i8, font_size: c_float, spacing: c_float) -> CVector2;
    fn GetCharPressed() -> c_int;
    fn IsKeyPressed(key: c_int) -> bool;
    fn IsKeyPressedRepeat(key: c_int) -> bool;
}

/*==========================================
             The public interface.
  ==========================================*/

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    fn to_ccolor(&self) -> CColor {
        return CColor {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        };
    }
}

pub mod colors {
    use super::Color;

    pub static LIGHT_GRAY: Color  = Color {r: 200, g: 200, b: 200, a: 255};
    pub static GRAY: Color        = Color {r: 130, g: 130, b: 130, a: 255};
    pub static DARK_GRAY: Color   = Color {r: 80, g: 80, b: 80, a: 255};
    pub static YELLOW: Color      = Color {r: 253, g: 249, b: 0, a: 255};
    pub static GOLD: Color        = Color {r: 255, g: 203, b: 0, a: 255};
    pub static ORANGE: Color      = Color {r: 255, g: 161, b: 0, a: 255};
    pub static PINK: Color        = Color {r: 255, g: 109, b: 194, a: 255};
    pub static RED: Color         = Color {r: 230, g: 41, b: 55, a: 255};
    pub static MAROON: Color      = Color {r: 190, g: 33, b: 55, a: 255};
    pub static GREEN: Color       = Color {r: 0, g: 228, b: 48, a: 255};
    pub static LIME: Color        = Color {r: 0, g: 158, b: 47, a: 255};
    pub static DARK_GREEN: Color  = Color {r: 0, g: 117, b: 44, a: 255};
    pub static SKY_BLUE: Color    = Color {r: 102, g: 191, b: 255, a: 255};
    pub static BLUE: Color        = Color {r: 0, g: 121, b: 241, a: 255};
    pub static DARK_BLUE: Color   = Color {r: 0, g: 82, b: 172, a: 255};
    pub static PURPLE: Color      = Color {r: 200, g: 122, b: 255, a: 255};
    pub static VIOLET: Color      = Color {r: 135, g: 60, b: 190, a: 255};
    pub static DARK_PURPLE: Color = Color {r: 112, g: 31, b: 126, a: 255};
    pub static BEIGE: Color       = Color {r: 211, g: 176, b: 131, a: 255};
    pub static BROWN: Color       = Color {r: 127, g: 106, b: 79, a: 255};
    pub static DARK_BROWN: Color  = Color {r: 76, g: 63, b: 47, a: 255};

    pub static WHITE: Color       = Color {r: 255, g: 255, b: 255, a: 255};
    pub static BLACK: Color       = Color {r: 0, g: 0, b: 0, a: 255};
    pub static BLANK: Color       = Color {r: 0, g: 0, b: 0, a: 0};
    pub static MAGENTA: Color     = Color {r: 255, g: 0, b: 255, a: 255};
    pub static RAY_WHITE: Color   = Color {r: 245, g: 245, b: 245, a: 255};
}

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    fn to_cvector2(&self) -> CVector2 {
        return CVector2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn from_angle_and_len(angle: f32, len: f32) -> Vector2 {
        return Vector2 {
            x: len * f32::cos(angle),
            y: len * f32::sin(angle),
        }
    }

    pub fn len(&self) -> f32 {
        return f32::sqrt(self.x * self.x + self.y * self.y);
    }

    pub fn normalize(&mut self) -> Vector2 {
        let len = self.len();
        self.x /= len;
        self.y /= len;

        return Vector2 {x: self.x, y: self.y};
    }

    pub fn rotate(&mut self, angle: f32) -> Vector2 {
        let cosres = f32::cos(angle);
        let sinres = f32::sin(angle);

        let mut result = Vector2 {x:0.0,y:0.0};

        result.x = self.x*cosres - self.y*sinres;
        result.y = self.x*sinres + self.y*cosres;

        self.x = result.x;
        self.y = result.y;

        return Vector2 {x: self.x, y: self.y};
    }

    pub fn dot(&self, rhs: Vector2) -> f32 {
        let result = self.x * rhs.x + self.y * rhs.y;

        return result;
    }

    pub fn det(&self, rhs: Vector2) -> f32 {
        let result = self.x * rhs.y - self.y * rhs.x;

        return result;
    }

    pub fn angle_with(&self, rhs: Vector2) -> f32 {
        let result;

        let dot = self.x * rhs.x + self.y * rhs.y;
        let det = self.x * rhs.y - self.y * rhs.x;

        result = f32::atan2(det, dot);

        return result;
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x + rhs,
            y: self.y + rhs,
        };
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        };
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        };
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        return Vector2 {x: self.x + rhs.x, y: self.y + rhs.y};
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        return Vector2 {x: self.x - rhs.x, y: self.y - rhs.y};
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        return Vector2 {x: self.x * rhs, y: self.y * rhs};
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Vector2 {
        return Vector2 {x: self.x / rhs, y: self.y / rhs};
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Vector2 {
        return Vector2 {x: self.x / rhs.x, y: self.y / rhs.y};
    }
}

pub struct WindowContext {}

impl WindowContext {
    pub fn window_should_close(&self) -> bool {
        let flag;

        unsafe {
            flag = WindowShouldClose();
        }

        return flag;
    }

    pub fn init_drawing_context(&self) -> DrawingContext {
        unsafe {
            BeginDrawing();
        }

        return DrawingContext {};
    }
}

impl Drop for WindowContext {
    fn drop(&mut self) {
        unsafe{
            CloseWindow();
        }
    }
}

pub struct DrawingContext {}

impl DrawingContext {
    pub fn clear_background(&mut self, color: Color) {
        unsafe {
            ClearBackground(color.to_ccolor());
        }
    }

    pub fn draw_text(&mut self, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
        unsafe {
            let converted_text = CString::new(text).expect("Failed to create CString.");
            let text_pointer = converted_text.as_ptr();

            let converted_color = color.to_ccolor();

            DrawText(text_pointer, pos_x as c_int, pos_y as c_int, font_size as c_int, converted_color);
        }
    }

    pub fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
        let converted_color = color.to_ccolor();

        unsafe {
            DrawRectangle(pos_x as c_int, pos_y as c_int, width as c_int, height as c_int, converted_color);
        }
    }

    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: f32, color: Color) {
        let converted_color = color.to_ccolor();

        unsafe {
            DrawCircle(center_x as c_int, center_y as c_int, radius as c_float, converted_color);
        }
    }

    pub fn draw_line_ex(&mut self, start_pos: Vector2, end_pos: Vector2, thickness: f32, color: Color) {
        let color = color.to_ccolor();
        let start_pos = start_pos.to_cvector2();
        let end_pos = end_pos.to_cvector2();
        
        unsafe {
            DrawLineEx(start_pos, end_pos, thickness as c_float, color);
        }
    }
}

impl Drop for DrawingContext {
    fn drop(&mut self) {
        unsafe {
            EndDrawing();
        }
    }
}

pub fn init_window_context(width: i32, height: i32, title: &str) -> WindowContext {
    if width < 0 || height < 0 {
        panic!("width and height should be non-negative.");
    }

    unsafe {
        InitWindow(width as c_int, height as c_int, CString::new(title).expect("Failed to create CString.").as_ptr());
    }

    return WindowContext {};
}

pub fn is_key_down(key: Key) -> bool {
    let flag;
    let converted_key = key.to_key_code();

    unsafe {
        flag = IsKeyDown(converted_key);
    }

    return flag;
}

pub fn get_char_pressed() -> Option<char> {
    let result;

    unsafe {
        result = GetCharPressed();
    }

    match result {
        0 => {
            return None;
        },
        c => {
            // Larger than all ascii.
            if c > 127 {
                return None;
            }

            return Some(c as u8 as char);
        },
    };
}

pub fn is_key_pressed(key: Key) -> bool {
    let result;
    let converted_key = key.to_key_code();

    unsafe {
        result = IsKeyPressed(converted_key);
    }

    return result;
}

pub fn is_key_pressed_repeat(key: Key) -> bool {
    let result;
    let converted_key = key.to_key_code();

    unsafe {
        result = IsKeyPressedRepeat(converted_key);
    }

    return result;
}

#[derive(Debug)]
pub enum Key {
    NULL,                       // Key: NULL, used for no key pressed
    // Alphanumeric keys
    APOSTROPHE,                 // Key: '
    COMMA,                      // Key: ,
    MINUS,                      // Key: -
    PERIOD,                     // Key: .
    SLASH,                      // Key: /
    ZERO,                       // Key: 0
    ONE,                        // Key: 1
    TWO,                        // Key: 2
    THREE,                      // Key: 3
    FOUR,                       // Key: 4
    FIVE,                       // Key: 5
    SIX,                        // Key: 6
    SEVEN,                      // Key: 7
    EIGHT,                      // Key: 8
    NINE,                       // Key: 9
    SEMICOLON,                  // Key: ;
    EQUAL,                      // Key: =
    A,                          // Key: A | a
    B,                          // Key: B | b
    C,                          // Key: C | c
    D,                          // Key: D | d
    E,                          // Key: E | e
    F,                          // Key: F | f
    G,                          // Key: G | g
    H,                          // Key: H | h
    I,                          // Key: I | i
    J,                          // Key: J | j
    K,                          // Key: K | k
    L,                          // Key: L | l
    M,                          // Key: M | m
    N,                          // Key: N | n
    O,                          // Key: O | o
    P,                          // Key: P | p
    Q,                          // Key: Q | q
    R,                          // Key: R | r
    S,                          // Key: S | s
    T,                          // Key: T | t
    U,                          // Key: U | u
    V,                          // Key: V | v
    W,                          // Key: W | w
    X,                          // Key: X | x
    Y,                          // Key: Y | y
    Z,                          // Key: Z | z
    LEFT_BRACKET,               // Key: [
    BACKSLASH,                  // Key: '\'
    RIGHT_BRACKET,              // Key: ]
    GRAVE,                      // Key: `
    // Function keys
    SPACE,                      // Key: Space
    ESCAPE,                     // Key: Esc
    ENTER,                      // Key: Enter
    TAB,                        // Key: Tab
    BACKSPACE,                  // Key: Backspace
    INSERT,                     // Key: Ins
    DELETE,                     // Key: Del
    RIGHT,                      // Key: Cursor right
    LEFT,                       // Key: Cursor left
    DOWN,                       // Key: Cursor down
    UP,                         // Key: Cursor up
    PAGE_UP,                    // Key: Page up
    PAGE_DOWN,                  // Key: Page down
    HOME,                       // Key: Home
    END,                        // Key: End
    CAPS_LOCK,                  // Key: Caps lock
    SCROLL_LOCK,                // Key: Scroll down
    NUM_LOCK,                   // Key: Num lock
    PRINT_SCREEN,               // Key: Print screen
    PAUSE,                      // Key: Pause
    F1,                         // Key: F1
    F2,                         // Key: F2
    F3,                         // Key: F3
    F4,                         // Key: F4
    F5,                         // Key: F5
    F6,                         // Key: F6
    F7,                         // Key: F7
    F8,                         // Key: F8
    F9,                         // Key: F9
    F10,                        // Key: F10
    F11,                        // Key: F11
    F12,                        // Key: F12
    LEFT_SHIFT,                 // Key: Shift left
    LEFT_CONTROL,               // Key: Control left
    LEFT_ALT,                   // Key: Alt left
    LEFT_SUPER,                 // Key: Super left
    RIGHT_SHIFT,                // Key: Shift right
    RIGHT_CONTROL,              // Key: Control right
    RIGHT_ALT,                  // Key: Alt right
    RIGHT_SUPER,                // Key: Super right
    KB_MENU,                    // Key: KB menu
    // Keypad keys
    KP_0,                       // Key: Keypad 0
    KP_1,                       // Key: Keypad 1
    KP_2,                       // Key: Keypad 2
    KP_3,                       // Key: Keypad 3
    KP_4,                       // Key: Keypad 4
    KP_5,                       // Key: Keypad 5
    KP_6,                       // Key: Keypad 6
    KP_7,                       // Key: Keypad 7
    KP_8,                       // Key: Keypad 8
    KP_9,                       // Key: Keypad 9
    KP_DECIMAL,                 // Key: Keypad .
    KP_DIVIDE,                  // Key: Keypad /
    KP_MULTIPLY,                // Key: Keypad *
    KP_SUBTRACT,                // Key: Keypad -
    KP_ADD,                     // Key: Keypad +
    KP_ENTER,                   // Key: Keypad Enter
    KP_EQUAL,                   // Key: Keypad =
    // Android key buttons
    BACK,                       // Key: Android back button
    MENU,                       // Key: Android menu button
    VOLUME_UP,                  // Key: Android volume up button
    VOLUME_DOWN,                // Key: Android volume down button
}

impl Key {
    fn to_key_code(&self) -> c_int {
        match self {
            Key::NULL            => 0,        // Key: NULL, used for no key pressed
            // Alphanumeric keys
            Key::APOSTROPHE      => 39,       // Key: '
            Key::COMMA           => 44,       // Key: ,
            Key::MINUS           => 45,       // Key: -
            Key::PERIOD          => 46,       // Key: .
            Key::SLASH           => 47,       // Key: /
            Key::ZERO            => 48,       // Key: 0
            Key::ONE             => 49,       // Key: 1
            Key::TWO             => 50,       // Key: 2
            Key::THREE           => 51,       // Key: 3
            Key::FOUR            => 52,       // Key: 4
            Key::FIVE            => 53,       // Key: 5
            Key::SIX             => 54,       // Key: 6
            Key::SEVEN           => 55,       // Key: 7
            Key::EIGHT           => 56,       // Key: 8
            Key::NINE            => 57,       // Key: 9
            Key::SEMICOLON       => 59,       // Key: ;
            Key::EQUAL           => 61,       // Key: =
            Key::A               => 65,       // Key: A | a
            Key::B               => 66,       // Key: B | b
            Key::C               => 67,       // Key: C | c
            Key::D               => 68,       // Key: D | d
            Key::E               => 69,       // Key: E | e
            Key::F               => 70,       // Key: F | f
            Key::G               => 71,       // Key: G | g
            Key::H               => 72,       // Key: H | h
            Key::I               => 73,       // Key: I | i
            Key::J               => 74,       // Key: J | j
            Key::K               => 75,       // Key: K | k
            Key::L               => 76,       // Key: L | l
            Key::M               => 77,       // Key: M | m
            Key::N               => 78,       // Key: N | n
            Key::O               => 79,       // Key: O | o
            Key::P               => 80,       // Key: P | p
            Key::Q               => 81,       // Key: Q | q
            Key::R               => 82,       // Key: R | r
            Key::S               => 83,       // Key: S | s
            Key::T               => 84,       // Key: T | t
            Key::U               => 85,       // Key: U | u
            Key::V               => 86,       // Key: V | v
            Key::W               => 87,       // Key: W | w
            Key::X               => 88,       // Key: X | x
            Key::Y               => 89,       // Key: Y | y
            Key::Z               => 90,       // Key: Z | z
            Key::LEFT_BRACKET    => 91,       // Key: [
            Key::BACKSLASH       => 92,       // Key: '\'
            Key::RIGHT_BRACKET   => 93,       // Key: ]
            Key::GRAVE           => 96,       // Key: `
            // Function keys
            Key::SPACE           => 32,       // Key: Space
            Key::ESCAPE          => 256,      // Key: Esc
            Key::ENTER           => 257,      // Key: Enter
            Key::TAB             => 258,      // Key: Tab
            Key::BACKSPACE       => 259,      // Key: Backspace
            Key::INSERT          => 260,      // Key: Ins
            Key::DELETE          => 261,      // Key: Del
            Key::RIGHT           => 262,      // Key: Cursor right
            Key::LEFT            => 263,      // Key: Cursor left
            Key::DOWN            => 264,      // Key: Cursor down
            Key::UP              => 265,      // Key: Cursor up
            Key::PAGE_UP         => 266,      // Key: Page up
            Key::PAGE_DOWN       => 267,      // Key: Page down
            Key::HOME            => 268,      // Key: Home
            Key::END             => 269,      // Key: End
            Key::CAPS_LOCK       => 280,      // Key: Caps lock
            Key::SCROLL_LOCK     => 281,      // Key: Scroll down
            Key::NUM_LOCK        => 282,      // Key: Num lock
            Key::PRINT_SCREEN    => 283,      // Key: Print screen
            Key::PAUSE           => 284,      // Key: Pause
            Key::F1              => 290,      // Key: F1
            Key::F2              => 291,      // Key: F2
            Key::F3              => 292,      // Key: F3
            Key::F4              => 293,      // Key: F4
            Key::F5              => 294,      // Key: F5
            Key::F6              => 295,      // Key: F6
            Key::F7              => 296,      // Key: F7
            Key::F8              => 297,      // Key: F8
            Key::F9              => 298,      // Key: F9
            Key::F10             => 299,      // Key: F10
            Key::F11             => 300,      // Key: F11
            Key::F12             => 301,      // Key: F12
            Key::LEFT_SHIFT      => 340,      // Key: Shift left
            Key::LEFT_CONTROL    => 341,      // Key: Control left
            Key::LEFT_ALT        => 342,      // Key: Alt left
            Key::LEFT_SUPER      => 343,      // Key: Super left
            Key::RIGHT_SHIFT     => 344,      // Key: Shift right
            Key::RIGHT_CONTROL   => 345,      // Key: Control right
            Key::RIGHT_ALT       => 346,      // Key: Alt right
            Key::RIGHT_SUPER     => 347,      // Key: Super right
            Key::KB_MENU         => 348,      // Key: KB menu
            // Keypad keys
            Key::KP_0            => 320,      // Key: Keypad 0
            Key::KP_1            => 321,      // Key: Keypad 1
            Key::KP_2            => 322,      // Key: Keypad 2
            Key::KP_3            => 323,      // Key: Keypad 3
            Key::KP_4            => 324,      // Key: Keypad 4
            Key::KP_5            => 325,      // Key: Keypad 5
            Key::KP_6            => 326,      // Key: Keypad 6
            Key::KP_7            => 327,      // Key: Keypad 7
            Key::KP_8            => 328,      // Key: Keypad 8
            Key::KP_9            => 329,      // Key: Keypad 9
            Key::KP_DECIMAL      => 330,      // Key: Keypad .
            Key::KP_DIVIDE       => 331,      // Key: Keypad /
            Key::KP_MULTIPLY     => 332,      // Key: Keypad *
            Key::KP_SUBTRACT     => 333,      // Key: Keypad -
            Key::KP_ADD          => 334,      // Key: Keypad +
            Key::KP_ENTER        => 335,      // Key: Keypad Enter
            Key::KP_EQUAL        => 336,      // Key: Keypad =
            // Android key buttons
            Key::BACK            => 4,        // Key: Android back button
            Key::MENU            => 5,        // Key: Android menu button
            Key::VOLUME_UP       => 24,       // Key: Android volume up button
            Key::VOLUME_DOWN     => 25,       // Key: Android volume down button
        }
    }
}

pub fn get_delta_time() -> f32 {
    let out;

    unsafe {
        out = GetFrameTime();
    }

    return out as f32;
}

pub fn get_mouse_position() -> Vector2 {
    let out;

    unsafe {
        out = GetMousePosition();
    }

    let converted_out = Vector2 {x: out.x, y: out.y};

    return converted_out;
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl MouseButton {
    fn to_button_code(&self) -> c_int {
        match self {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            MouseButton::Middle => 2,
        }
    }
}

pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    let result;

    let converted_button = button.to_button_code();

    unsafe {
        result = IsMouseButtonPressed(converted_button);
    }

    return result;
}

pub fn get_screen_width() -> i32 {
    let result;

    unsafe {
        result = GetScreenWidth();
    }

    return result;
}

pub fn get_screen_height() -> i32 {
    let result;

    unsafe {
        result = GetScreenHeight();
    }

    return result;
}

pub fn get_fps() -> i32 {
    let result;

    unsafe {
        result = GetFPS();
    }

    return result;
}

/*===================================
             Font stuff.
=====================================*/

pub struct Font {
    inner: CFont,
}

impl Font {
    fn to_cfont(&self) -> CFont {
        return self.inner.clone();
    }
}

pub fn get_default_font() -> Font {
    let result: CFont;

    unsafe {
        result = GetFontDefault();
    }

    return Font { inner: result };
}

pub fn set_text_line_spacing(spacing: i32) {
    unsafe {
        SetTextLineSpacing(spacing as c_int);
    }
}

pub fn measure_text_ex(font: Font, text: &str, font_size: f32, spacing: f32) -> Vector2 {
    let converted_text = CString::new(text).expect("Failed to create CString.");
    let text_pointer = converted_text.as_ptr();
    let result: CVector2;

    unsafe {
        result = MeasureTextEx(font.to_cfont(), text_pointer, font_size as c_float, spacing as c_float);
    }

    return Vector2 { x: result.x, y: result.y };
}
