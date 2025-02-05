use windows_sys::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

pub fn new() {
    unsafe {
        let instance = GetModuleHandleA(std::ptr::null());
        debug_assert!(!instance.is_null());

        let hoshi_window_classname = w!("window");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW, // Style: https://learn.microsoft.com/ru-ru/windows/win32/winmsg/window-class-styles
            lpfnWndProc: Some(wndproc), // Window procedure: https://learn.microsoft.com/ru-ru/windows/win32/api/winuser/nc-winuser-wndproc
            cbClsExtra: 0, // Extra bytes to allocate at the beginning of the class structure
            cbWndExtra: 0, // Extra bytes to allocate after the window structure
            hInstance: instance, // Window instance descriptor
            hIcon: core::ptr::null_mut(), // Icon descriptor,
            hCursor: LoadCursorW(core::ptr::null_mut(), IDC_ARROW), // Custor descriptor
            hbrBackground: core::ptr::null_mut(), // Background brust descriptor ??
            lpszMenuName: std::ptr::null(), // "Resource name" ??
            lpszClassName: hoshi_window_classname, // Class name
        };
        let atom = RegisterClassW(&wc);

        debug_assert!(atom != 0);

        let w = CreateWindowExW(
            0, // Extended widnow styles:  https://learn.microsoft.com/ru-ru/windows/win32/winmsg/extended-window-styles
            hoshi_window_classname, // Class name
            w!("HoshiBoshi"), // Window name
            WS_OVERLAPPEDWINDOW | WS_VISIBLE, // Window styles
            CW_USEDEFAULT, // X
            CW_USEDEFAULT, // Y
            CW_USEDEFAULT, // Width
            CW_USEDEFAULT, // Height
            core::ptr::null_mut(), // Parent window handle
            core::ptr::null_mut(), // Menu handle ??
            instance, // Instance handle
            std::ptr::null(), // ? hz
        );

        let mut message = std::mem::zeroed();

        while GetMessageA(&mut message, core::ptr::null_mut(), 0, 0) != 0 {
            DispatchMessageA(&message);
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, std::ptr::null());
                0
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
