/*
[package]
name = "t005"
version = "0.1.0"
edition = "2021"

[dependencies]
druid = "0.8.3"
*/
use druid::widget::prelude::*;
use druid::widget::{Label, Flex};
use druid::{AppLauncher, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_root_widget())
        .title("Floating Window")
        .window_size((200.0, 100.0)); // 设置窗口大小

    AppLauncher::with_window(main_window)
        .launch(())?;
    
    Ok(())
}

fn build_root_widget() -> impl Widget<()> {
    Flex::column()
        .with_child(Label::new("这是一个悬浮框"))
}
