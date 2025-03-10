use bytesize::ByteSize;
use iced::{
    system,
    widget::{column, text},
    Element,
};

use crate::gui::app::app::Message;

pub fn system_information_screen(information: system::Information) -> Element<'static, Message> {
    let system_name = text(format!(
        "System name: {}",
        information
            .system_name
            .as_ref()
            .unwrap_or(&"unknown".to_string())
    ));

    let system_kernel = text(format!(
        "System kernel: {}",
        information
            .system_kernel
            .as_ref()
            .unwrap_or(&"unknown".to_string())
    ));

    let system_version = text(format!(
        "System version: {}",
        information
            .system_version
            .as_ref()
            .unwrap_or(&"unknown".to_string())
    ));

    let system_short_version = text(format!(
        "System short version: {}",
        information
            .system_short_version
            .as_ref()
            .unwrap_or(&"unknown".to_string())
    ));

    let cpu_brand = text(format!("Processor brand: {}", information.cpu_brand));

    let cpu_cores = text(format!(
        "Processor cores: {}",
        information
            .cpu_cores
            .map_or("unknown".to_string(), |cores| cores.to_string())
    ));

    let memory_readable = ByteSize::b(information.memory_total).to_string();

    let memory_total = text(format!(
        "Memory (total): {} bytes ({memory_readable})",
        information.memory_total,
    ));

    let memory_text = if let Some(memory_used) = information.memory_used {
        let memory_readable = ByteSize::b(memory_used).to_string();

        format!("{memory_used} bytes ({memory_readable})")
    } else {
        String::from("None")
    };

    let memory_used = text(format!("Memory (used): {memory_text}"));

    let graphics_adapter = text(format!(
        "Graphics adapter: {}",
        information.graphics_adapter
    ));

    let graphics_backend = text(format!(
        "Graphics backend: {}",
        information.graphics_backend
    ));

    column![
        system_name.size(20),
        system_kernel.size(20),
        system_version.size(20),
        system_short_version.size(20),
        cpu_brand.size(20),
        cpu_cores.size(20),
        memory_total.size(20),
        memory_used.size(20),
        graphics_adapter.size(20),
        graphics_backend.size(20),
        // button("Refresh").on_press(Message::Refresh)
    ]
    .spacing(10)
    .into()
}
