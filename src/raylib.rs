use std::ffi::{c_char, CString};

use crate::{functions::IntCallable, value::Value, IntError};

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum KeyboardKey {
    KEY_NULL = 0,
    KEY_APOSTROPHE = 39,
    KEY_COMMA = 44,
    KEY_MINUS = 45,
    KEY_PERIOD = 46,
    KEY_SLASH = 47,
    KEY_ZERO = 48,
    KEY_ONE = 49,
    KEY_TWO = 50,
    KEY_THREE = 51,
    KEY_FOUR = 52,
    KEY_FIVE = 53,
    KEY_SIX = 54,
    KEY_SEVEN = 55,
    KEY_EIGHT = 56,
    KEY_NINE = 57,
    KEY_SEMICOLON = 59,
    KEY_EQUAL = 61,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    KEY_SPACE = 32,
    KEY_ESCAPE = 256,
    KEY_ENTER = 257,
    KEY_TAB = 258,
    KEY_BACKSPACE = 259,
    KEY_INSERT = 260,
    KEY_DELETE = 261,
    KEY_RIGHT = 262,
    KEY_LEFT = 263,
    KEY_DOWN = 264,
    KEY_UP = 265,
    KEY_PAGE_UP = 266,
    KEY_PAGE_DOWN = 267,
    KEY_HOME = 268,
    KEY_END = 269,
    KEY_CAPS_LOCK = 280,
    KEY_SCROLL_LOCK = 281,
    KEY_NUM_LOCK = 282,
    KEY_PRINT_SCREEN = 283,
    KEY_PAUSE = 284,
    KEY_F1 = 290,
    KEY_F2 = 291,
    KEY_F3 = 292,
    KEY_F4 = 293,
    KEY_F5 = 294,
    KEY_F6 = 295,
    KEY_F7 = 296,
    KEY_F8 = 297,
    KEY_F9 = 298,
    KEY_F10 = 299,
    KEY_F11 = 300,
    KEY_F12 = 301,
    KEY_LEFT_SHIFT = 340,
    KEY_LEFT_CONTROL = 341,
    KEY_LEFT_ALT = 342,
    KEY_LEFT_SUPER = 343,
    KEY_RIGHT_SHIFT = 344,
    KEY_RIGHT_CONTROL = 345,
    KEY_RIGHT_ALT = 346,
    KEY_RIGHT_SUPER = 347,
    KEY_KB_MENU = 348,
    KEY_LEFT_BRACKET = 91,
    KEY_BACKSLASH = 92,
    KEY_RIGHT_BRACKET = 93,
    KEY_GRAVE = 96,
    KEY_KP_0 = 320,
    KEY_KP_1 = 321,
    KEY_KP_2 = 322,
    KEY_KP_3 = 323,
    KEY_KP_4 = 324,
    KEY_KP_5 = 325,
    KEY_KP_6 = 326,
    KEY_KP_7 = 327,
    KEY_KP_8 = 328,
    KEY_KP_9 = 329,
    KEY_KP_DECIMAL = 330,
    KEY_KP_DIVIDE = 331,
    KEY_KP_MULTIPLY = 332,
    KEY_KP_SUBTRACT = 333,
    KEY_KP_ADD = 334,
    KEY_KP_ENTER = 335,
    KEY_KP_EQUAL = 336,
    KEY_BACK = 4,
    KEY_VOLUME_UP = 24,
    KEY_VOLUME_DOWN = 25,
}

pub struct InitWindow;
impl IntCallable for InitWindow {
    fn arity(&self) -> usize {
        3
    }

    fn name(&self) -> String {
        String::from("<fun InitWindow>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn InitWindow(width: i32, height: i32, title: *const c_char);
        }
        let width = arguments[0].double().unwrap() as i32;
        let height = arguments[1].double().unwrap() as i32;
        let title = CString::new(arguments.get(2).unwrap().clone().str().unwrap()).unwrap();
        unsafe {
            InitWindow(width, height, title.as_ptr());
        }
        Ok(Value::Nil)
    }
}

pub struct SetTargetFPS;
impl IntCallable for SetTargetFPS {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun SetTargetFPS>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn SetTargetFPS(fps: i32);
        }
        let fps = arguments[0].double().unwrap() as i32;
        unsafe {
            SetTargetFPS(fps);
        }
        Ok(Value::Nil)
    }
}

pub struct BeginDrawing;
impl IntCallable for BeginDrawing {
    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> String {
        String::from("<fun BeginDrawing>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        _: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn BeginDrawing();
        }
        unsafe {
            BeginDrawing();
        }
        Ok(Value::Nil)
    }
}

pub struct EndDrawing;
impl IntCallable for EndDrawing {
    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> String {
        String::from("<fun EndDrawing>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        _: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn EndDrawing();
        }
        unsafe {
            EndDrawing();
        }
        Ok(Value::Nil)
    }
}

pub struct ClearBackground;
impl IntCallable for ClearBackground {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun ClearBackground>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn ClearBackground(color: u32);
        }
        let color = arguments[0].double().unwrap() as u32;
        unsafe {
            ClearBackground(color);
        }
        Ok(Value::Nil)
    }
}

pub struct DrawText;
impl IntCallable for DrawText {
    fn arity(&self) -> usize {
        5
    }

    fn name(&self) -> String {
        String::from("<fun DrawText>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn DrawText(text: *const c_char, posX: i32, posY: i32, fontSize: i32, color: u32);
        }
        // TODO: I don't need to do that, I can just pass a refernece somehow
        let text = CString::new(arguments.get(0).unwrap().clone().str().unwrap()).unwrap();
        let pos_x = arguments[1].double().unwrap() as i32;
        let pos_y = arguments[2].double().unwrap() as i32;
        let font_size = arguments[3].double().unwrap() as i32;
        let color = arguments[4].double().unwrap() as u32;
        unsafe {
            DrawText(text.as_ptr(), pos_x, pos_y, font_size, color);
        }
        Ok(Value::Nil)
    }
}

pub struct WindowShouldClose;
impl IntCallable for WindowShouldClose {
    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> String {
        String::from("<fun WindowShouldClose>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        _: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn WindowShouldClose() -> bool;
        }
        let result = unsafe { WindowShouldClose() };
        Ok(Value::Bool(result))
    }
}

pub struct DrawRectangle;
impl IntCallable for DrawRectangle {
    fn arity(&self) -> usize {
        5
    }

    fn name(&self) -> String {
        String::from("<fun DrawRectangle>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn DrawRectangle(posX: i32, posY: i32, width: i32, height: i32, color: u32) -> bool;
        }

        let pos_x = arguments[0].double().unwrap() as i32;
        let pos_y = arguments[1].double().unwrap() as i32;
        let width = arguments[2].double().unwrap() as i32;
        let height = arguments[3].double().unwrap() as i32;
        let color = arguments[4].double().unwrap() as u32;
        unsafe {
            DrawRectangle(pos_x, pos_y, width, height, color);
        }
        Ok(Value::Nil)
    }
}

pub struct GetFrameTime;
impl IntCallable for GetFrameTime {
    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> String {
        String::from("<fun GetFrameTime>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        _: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn GetFrameTime() -> f32;
        }
        let result = unsafe { GetFrameTime() };
        Ok(Value::Double(result as f64))
    }
}

pub struct DrawFPS;
impl IntCallable for DrawFPS {
    fn arity(&self) -> usize {
        2
    }

    fn name(&self) -> String {
        String::from("<fun DrawFPS>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn DrawFPS(posX: i32, posY: i32);
        }
        let pos_x = arguments[0].double().unwrap() as i32;
        let pos_y = arguments[1].double().unwrap() as i32;
        unsafe {
            DrawFPS(pos_x, pos_y);
        }
        Ok(Value::Nil)
    }
}

pub struct IsKeyDown;
impl IntCallable for IsKeyDown {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun IsKeyDown>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        extern "C" {
            fn IsKeyDown(key: u32) -> bool;
        }
        let key = arguments[0].double().unwrap() as u32;
        let result = unsafe { IsKeyDown(key) };
        Ok(Value::Bool(result))
    }
}

pub struct CheckCollisionRecs;
impl IntCallable for CheckCollisionRecs {
    fn arity(&self) -> usize {
        2
    }

    fn name(&self) -> String {
        String::from("<fun CheckCollisionRecs>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        #[repr(C)]
        pub struct Rectangle {
            pub x: f32,
            pub y: f32,
            pub width: f32,
            pub height: f32,
        }
        extern "C" {
            fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
        }
        let rec1 = arguments[0].structure().unwrap().borrow();
        let rec1 = Rectangle {
            x: rec1.get("x").unwrap().double().unwrap() as f32,
            y: rec1["y"].double().unwrap() as f32,
            width: rec1["width"].double().unwrap() as f32,
            height: rec1["height"].double().unwrap() as f32,
        };
        let rec2 = arguments[1].structure().unwrap().borrow();
        let rec2 = Rectangle {
            x: rec2["x"].double().unwrap() as f32,
            y: rec2["y"].double().unwrap() as f32,
            width: rec2["width"].double().unwrap() as f32,
            height: rec2["height"].double().unwrap() as f32,
        };
        let result = unsafe { CheckCollisionRecs(rec1, rec2) };
        Ok(Value::Bool(result))
    }
}

pub struct DrawRectangleRec;
impl IntCallable for DrawRectangleRec {
    fn arity(&self) -> usize {
        2
    }

    fn name(&self) -> String {
        String::from("<fun DrawRectangleRec>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        #[repr(C)]
        pub struct Rectangle {
            pub x: f32,
            pub y: f32,
            pub width: f32,
            pub height: f32,
        }
        extern "C" {
            fn DrawRectangleRec(rec: Rectangle, color: u32);
        }
        let rec = arguments[0].structure().unwrap().borrow();
        let rec = Rectangle {
            x: rec.get("x").unwrap().double().unwrap() as f32,
            y: rec["y"].double().unwrap() as f32,
            width: rec["width"].double().unwrap() as f32,
            height: rec["height"].double().unwrap() as f32,
        };
        let color = arguments[1].double().unwrap() as u32;
        unsafe {
            DrawRectangleRec(rec, color);
        };
        Ok(Value::Nil)
    }
}
