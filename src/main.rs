use winapi::ctypes::c_int;
use winapi::shared::minwindef::{DWORD, LPARAM, WORD, WPARAM};
use winapi::um::winuser;

const HOTKEY_VK: WORD = winuser::VK_F8 as WORD;
const SIMULATE_VK: WORD = winuser::VK_LWIN as WORD;

unsafe fn keyboard_input(vk: WORD, up: bool) -> winuser::INPUT {
    let mut u: winuser::INPUT_u = std::mem::zeroed();
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

fn main() {
    let hotkey_id: c_int = 1;
    unsafe {
        winuser::RegisterHotKey(std::ptr::null_mut(), hotkey_id, 0 as u32, HOTKEY_VK.into());
        let mut msg: winuser::MSG = std::mem::zeroed();
        let mut inputs = [
            keyboard_input(SIMULATE_VK, false),
            keyboard_input(SIMULATE_VK, true),
        ];
        loop {
            if winuser::GetMessageW(
                &mut msg as winuser::LPMSG,
                std::ptr::null_mut(),
                winuser::WM_HOTKEY,
                winuser::WM_HOTKEY,
            ) <= 0
            {
                break;
            }
            println!("lparam {} wparam {}", msg.lParam, msg.wParam);
            if msg.hwnd != std::ptr::null_mut()
                || msg.message != winuser::WM_HOTKEY
                || msg.lParam != (HOTKEY_VK as LPARAM) << 16
                || msg.wParam != hotkey_id as WPARAM
            {
                continue;
            }
            winuser::SendInput(
                inputs.len() as u32,
                &mut inputs[0] as winuser::LPINPUT,
                std::mem::size_of::<winuser::INPUT>() as i32,
            );
        }
    }
}
