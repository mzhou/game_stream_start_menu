use winapi::shared::minwindef::{DWORD, WORD};
use winapi::um::winuser;

unsafe fn keyboard_input(wVk: WORD, up: bool) -> winuser::INPUT {
    let mut u: winuser::INPUT_u = std::mem::zeroed();
    *u.ki_mut() = winuser::KEYBDINPUT {
        wVk,
        wScan: winuser::MapVirtualKeyA(wVk.into(), winuser::MAPVK_VK_TO_VSC) as WORD,
        dwFlags: winuser::KEYEVENTF_EXTENDEDKEY | ((up as DWORD) * winuser::KEYEVENTF_KEYUP),
        time: 0,
        dwExtraInfo: winuser::GetMessageExtraInfo() as usize,
    };
    winuser::INPUT {
        type_: winuser::INPUT_KEYBOARD,
        u,
    }
}

fn main() {
    unsafe {
        winuser::RegisterHotKey(
            std::ptr::null_mut(),
            1,
            winuser::MOD_ALT as u32,
            winuser::VK_ESCAPE as u32,
        );
        let mut msg: winuser::MSG = std::mem::zeroed();
        let mut inputs = [
            keyboard_input(winuser::VK_LWIN as WORD, false),
            keyboard_input(winuser::VK_LWIN as WORD, true),
        ];
        loop {
            winuser::GetMessageW(
                &mut msg as winuser::LPMSG,
                std::ptr::null_mut(),
                winuser::WM_HOTKEY,
                winuser::WM_HOTKEY,
            );
            println!("got message");
            winuser::SendInput(
                1,
                &mut inputs[0] as winuser::LPINPUT,
                std::mem::size_of::<winuser::INPUT>() as i32,
            );
            winuser::SendInput(
                1,
                &mut inputs[1] as winuser::LPINPUT,
                std::mem::size_of::<winuser::INPUT>() as i32,
            );
        }
    }
}
