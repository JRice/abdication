//! Dragging an exported file out of the panel into the DAW.
//!
//! On Windows (with the `drag` feature): OLE drag and drop, with the shell's
//! own data object for the file. DoDragDrop runs its own message loop until
//! the drop; called from the panel's event handler it would re-enter
//! baseview's window procedure while that's still busy with the click (a
//! RefCell already borrowed: a crash in the host). So the drag starts from a
//! zero-delay thread timer instead, which the host's message loop runs once
//! the click has been handled, outside baseview.
//!
//! DoDragDrop also eats the button-up that ends the drag, and baseview counts
//! buttons down to decide when to let go of the mouse: missing that one
//! release, it kept the mouse captured, so clicks meant for the DAW went to
//! the panel. So once the drag is over, the window that had the mouse gets
//! the release it missed. Elsewhere: not available.

use std::path::PathBuf;

/// Whether dragging works in this build.
pub const AVAILABLE: bool = cfg!(all(windows, feature = "drag"));

/// Start dragging `path` as soon as the current event has been handled.
/// False if it can't (then the file is just saved).
pub fn drag_file(path: PathBuf) -> bool {
    imp::drag_file(path)
}

#[cfg(all(windows, feature = "drag"))]
#[allow(non_snake_case)]
mod imp {
    use std::cell::RefCell;
    use std::path::PathBuf;

    use windows::core::{implement, HRESULT, HSTRING};
    use windows::Win32::Foundation::{
        BOOL, DRAGDROP_S_CANCEL, DRAGDROP_S_DROP, DRAGDROP_S_USEDEFAULTCURSORS, HWND, LPARAM, S_OK, WPARAM,
    };
    use windows::Win32::System::Com::IDataObject;
    use windows::Win32::System::Ole::{DoDragDrop, IDropSource, IDropSource_Impl, OleInitialize, DROPEFFECT, DROPEFFECT_COPY};
    use windows::Win32::System::SystemServices::{MK_LBUTTON, MODIFIERKEYS_FLAGS};
    use windows::Win32::UI::Shell::{BHID_DataObject, IShellItem, SHCreateItemFromParsingName};
    use windows::Win32::UI::Input::KeyboardAndMouse::GetCapture;
    use windows::Win32::UI::WindowsAndMessaging::{KillTimer, PostMessageW, SetTimer, WM_LBUTTONUP};

    thread_local! {
        static PENDING: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
    }

    /// The drag's source: carries on while the left button is down, drops on release.
    #[implement(IDropSource)]
    struct Source;

    impl IDropSource_Impl for Source_Impl {
        fn QueryContinueDrag(&self, escape: BOOL, keys: MODIFIERKEYS_FLAGS) -> HRESULT {
            if escape.as_bool() {
                DRAGDROP_S_CANCEL
            } else if (keys.0 & MK_LBUTTON.0) == 0 {
                DRAGDROP_S_DROP
            } else {
                S_OK
            }
        }

        fn GiveFeedback(&self, _effect: DROPEFFECT) -> HRESULT {
            DRAGDROP_S_USEDEFAULTCURSORS
        }
    }

    unsafe extern "system" fn start(_: HWND, _: u32, id: usize, _: u32) {
        let _ = KillTimer(HWND::default(), id);
        let Some(path) = PENDING.with(|p| p.borrow_mut().take()) else { return };
        // The panel's window, holding the mouse while the button is down.
        let window = GetCapture();
        let Ok(data) = file_data(&path) else { return };
        let source: IDropSource = Source.into();
        let mut effect = DROPEFFECT::default();
        let _ = DoDragDrop(&data, &source, DROPEFFECT_COPY, &mut effect);
        if !window.is_invalid() {
            let _ = PostMessageW(window, WM_LBUTTONUP, WPARAM(0), LPARAM(0));
        }
    }

    /// The shell's data object for the file, as Explorer would drag it.
    unsafe fn file_data(path: &std::path::Path) -> windows::core::Result<IDataObject> {
        // Already initialized by the host, as a rule; then this just says so.
        let _ = OleInitialize(None);
        let item: IShellItem = SHCreateItemFromParsingName(&HSTRING::from(path.as_os_str()), None)?;
        item.BindToHandler(None, &BHID_DataObject)
    }

    pub fn drag_file(path: PathBuf) -> bool {
        PENDING.with(|p| *p.borrow_mut() = Some(path));
        unsafe { SetTimer(HWND::default(), 0, 0, Some(start)) != 0 }
    }
}

#[cfg(not(all(windows, feature = "drag")))]
mod imp {
    pub fn drag_file(_: std::path::PathBuf) -> bool {
        false
    }
}
