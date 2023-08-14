extern crate wee_alloc;

use riscv_emu_rust::{default_terminal::DefaultTerminal, Emulator};
use roblox_rs::{println, *};
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// # Safety
#[allow(unreachable_code)]
pub fn main() {
    let container = Script::new();
    container.set_parent(&Some(&Workspace::instance()));
    println!("got workspace");
    let surface_gui = SurfaceGui::new();
    let part = Part::new();
    let textbox = TextBox::new();
    textbox.set_size(&UDim2::from_scale(1., 1.));
    textbox.set_font(&enums::Font::Code);
    textbox.set_text_x_alignment(&enums::TextXAlignment::Left);
    textbox.set_text_y_alignment(&enums::TextYAlignment::Top);
    textbox.set_text("linux emulator\n");
    textbox.set_text_size(30.);
    textbox.set_parent(&Some(&surface_gui));
    part.set_anchored(true);
    part.set_color(&Color3::from_rgb(0., 255., 0.));
    part.set_material(&enums::Material::Glass);
    part.set_size(&Vector3::new_with_position(16., 10., 0.05));
    part.set_position(&Vector3::new_with_position(0., 0., 0.));
    part.set_can_collide(false);
    part.set_anchored(true);
    part.set_can_query(false);
    surface_gui.set_face(&enums::NormalId::Back);
    surface_gui.set_adornee(&part);
    surface_gui.set_sizing_mode(&enums::SurfaceGuiSizingMode::PixelsPerStud);
    surface_gui.set_pixels_per_stud(100.);
    surface_gui.set_parent(&Some(&container));
    part.set_parent(&Some(&container));

    let terminal = DefaultTerminal::new();
    let mut emulator = Emulator::new(Box::new(terminal));
    println!("got http");
    let root_fs = HttpService::instance().get_async(
        "https://takahirox.github.io/riscv-rust/resources/linux/rootfs.img",
        false,
    );
    let fw_payload = HttpService::instance().get_async(
        "https://takahirox.github.io/riscv-rust/resources/linux/opensbi/fw_payload.elf",
        false,
    );
    emulator.setup_program(fw_payload.as_bytes().to_vec());
    emulator.setup_filesystem(root_fs.as_bytes().to_vec());
    emulator.run();
    println!("got runservice");
    RunService::instance().on_heartbeat(move |_| {
        let t = emulator.get_mut_terminal();
        let byte = t.get_output();
        let mut string = textbox.text();
        if byte > 0 {
            string.push_str(std::str::from_utf8(&[byte]).unwrap_or(""));
            textbox.set_text(&string);
        }
    });
}
