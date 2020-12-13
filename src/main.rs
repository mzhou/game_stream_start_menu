#![feature(lang_items, start)]
#![no_std]

use core::mem::zeroed;
use core::ptr::null_mut;

use winapi::ctypes::c_int;
use winapi::shared::minwindef::{DWORD, LPARAM, WORD, WPARAM};
use winapi::um::winuser;

const HOTKEY_VK: WORD = winuser::VK_CAPITAL as WORD;
const SIMULATE_VK: WORD = winuser::VK_LWIN as WORD;

unsafe fn keyboard_input(vk: WORD, up: bool) -> winuser::INPUT {
    let mut u: winuser::INPUT_u = zeroed();
    *u.ki_mut() = winuser::KEYBDINPUT {
        wVk: vk,
        wScan: 0,
        dwFlags: ((up as DWORD) * winuser::KEYEVENTF_KEYUP),
        time: 0,
        dwExtraInfo: winuser::GetMessageExtraInfo() as usize,
    };
    winuser::INPUT {
        type_: winuser::INPUT_KEYBOARD,
        u,
    }
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let hotkey_id: c_int = 1;
    unsafe {
        winuser::RegisterHotKey(null_mut(), hotkey_id, 0 as u32, HOTKEY_VK.into());
        let mut msg: winuser::MSG = zeroed();
        let mut inputs = [
            keyboard_input(SIMULATE_VK, false),
            keyboard_input(SIMULATE_VK, true),
        ];
        loop {
            if winuser::GetMessageW(
                &mut msg as winuser::LPMSG,
                null_mut(),
                winuser::WM_HOTKEY,
                winuser::WM_HOTKEY,
            ) <= 0
            {
                break;
            }
            if msg.hwnd != null_mut()
                || msg.message != winuser::WM_HOTKEY
                || msg.lParam != (HOTKEY_VK as LPARAM) << 16
                || msg.wParam != hotkey_id as WPARAM
            {
                continue;
            }
            winuser::SendInput(
                inputs.len() as u32,
                &mut inputs[0] as winuser::LPINPUT,
                core::mem::size_of::<winuser::INPUT>() as i32,
            );
        }
    }
    0
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
