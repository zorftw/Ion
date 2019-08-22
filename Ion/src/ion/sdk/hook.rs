/// NOTE:
///     The use of "system" or "fastcall" is NOT arbitrary, in fact the reason of use for
///     fastcall in example the FSN hook, is because we might need to access the registers
///     provided


use crate::vmt::*;
use crate::ion::*;

use std::os::raw::{c_float, c_void};
use crate::ion::sdk::surface::Color;

type createmove_t    = unsafe extern "system" fn(_sampleframetime: c_float, *const sdk::definitions::cusercmd::CUserCmd) -> bool;
type fsn_t           = unsafe extern "fastcall" fn(ecx: *const c_void, edx: *const c_void, stage: i32);
type painttraverse_t = unsafe extern "fastcall" fn(exc: *const c_void, edx: *const c_void, panel: u32, force_repaint: bool, allow_force: bool);

pub fn hook() {
    let add_vmt = |vmt: VMT| { hooks.lock().unwrap().push(vmt); };

    let mut client_mode_vmt = VMT::new(interfaces.lock().unwrap().client_mode);
    let mut client_vmt      = VMT::new(interfaces.lock().unwrap().client);
    let mut panel_vmt       = VMT::new(interfaces.lock().unwrap().vgui_panel.base);

    client_mode_vmt.hook(24, create_move as _);
    client_vmt.hook(37, fsn as _);
    panel_vmt.hook(41, paint_traverse as _);

    add_vmt(client_mode_vmt);
    add_vmt(client_vmt);
    add_vmt(panel_vmt);
}

unsafe extern "system" fn create_move(_sampleframetime: c_float, cmd: *const sdk::definitions::cusercmd::CUserCmd) -> bool {

    if cmd.is_null() || cmd.read().command_number == 0 {
        std::mem::transmute::<_, createmove_t>(hooks.lock().unwrap()[0].get_original(24))(_sampleframetime, cmd);
    }

    false
}

unsafe extern "fastcall" fn fsn(exc: *const c_void, edx: *const c_void, stage: i32) {


    std::mem::transmute::<_, fsn_t>(hooks.lock().unwrap()[1].get_original(37))(exc, edx, stage);
}

unsafe extern "fastcall" fn paint_traverse(exc: *const c_void, edx: *const c_void, panel: u32, force_repaint: bool, allow_force: bool) {
    use std::ffi::CStr;

    let original = std::mem::transmute::<_, painttraverse_t>(hooks.lock().unwrap()[2].get_original(41));

    // Will be used for drawing
    static mut panel_id: u32 = 0;
    // Will be implemented later for no scope
    static mut panel_hud_id: u32 = 0;

    if panel_hud_id == 0 {
        let panel_name = interfaces.lock().unwrap().vgui_panel.get_panel_name(panel);

        let c_str = CStr::from_ptr(panel_name);
        let string = c_str.to_str().unwrap();

        if string.contains("HudZoom") {
            panel_hud_id = panel;
        }
    }

    if panel_id == 0 {
        let panel_name = interfaces.lock().unwrap().vgui_panel.get_panel_name(panel);

        let c_str = CStr::from_ptr(panel_name);
        let string = c_str.to_str().unwrap();

        if string.contains("MatSystemTopPanel") {
            panel_id = panel;
        }
    }

    original(exc, edx, panel, force_repaint, allow_force);

    // Top panel, so that we can draw :)
    if panel_id == panel {
        interfaces.lock().unwrap().vgui_surface.set_draw_color(Color::new_rgb(255, 0,0));
        interfaces.lock().unwrap().vgui_surface.draw_filled_rect(0, 0, 100, 100);
    }
}