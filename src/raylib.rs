use std::ffi::{c_char, CString};

use crate::{functions::IntCallable, value::Value, IntError};

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
