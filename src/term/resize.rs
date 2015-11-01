use libc;

use mioco;

use nix;
use nix::sys::signal;

use std::mem;
use std::os::unix;
use std::sync;

lazy_static! {
    static ref SIGNAL_CONTEXT: sync::RwLock<Option<SignalContext>> =
        sync::RwLock::new(None);
}

struct SignalContext {
    tty_fd: unix::io::RawFd,
    mailbox: mioco::MailboxOuterEnd<(usize, usize)>,
}

pub unsafe fn send_resizes_to(
    tty_fd: unix::io::RawFd,
    mailbox: mioco::MailboxOuterEnd<(usize, usize)>) -> Result<(), nix::Error> {

    *SIGNAL_CONTEXT.write().unwrap() = Some(SignalContext {
        tty_fd: tty_fd,
        mailbox: mailbox,
    });

    let resize_action = signal::SigAction::new(
        handle_resize, signal::SockFlag::empty(), signal::SigSet::empty());

    try!(signal::sigaction(signal::SIGWINCH, &resize_action));

    Ok(())
}

extern fn handle_resize(_: libc::c_int) {
    let context = SIGNAL_CONTEXT.read().unwrap();

    if let Some(ref c) = *context {
        let mut ws: ffi::WinSize = unsafe { mem::uninitialized() };
        unsafe {
            nix::from_ffi(ffi::ioctl(c.tty_fd, ffi::TIOCGWINSZ, &mut ws)).unwrap()
        };
        c.mailbox.send((ws.ws_col as usize, ws.ws_row as usize));
    }
}

mod ffi {
    use libc;

    #[cfg(target_os = "macos")]
    pub const TIOCGWINSZ: libc::c_ulong = 0x40087468;
    #[cfg(target_os = "linux")]
    pub const TIOCGWINSZ: libc::c_ulong = 0x00005413;

    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct WinSize {
        pub ws_row: u16,
        pub ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }

    extern {
        pub fn ioctl(fd: libc::c_int, req: libc::c_ulong, ...) -> libc::c_int;
    }
}
