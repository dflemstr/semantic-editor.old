use libc;

use mio;

use mioco;

use nix;
use nix::sys::signal;

use std::any;
use std::mem;
use std::os::unix;
use std::sync::atomic;

static RESIZED: atomic::AtomicBool = atomic::ATOMIC_BOOL_INIT;

pub fn send_resizes_to(
    tty_fd: unix::io::RawFd,
    size_mailbox: mioco::MailboxOuterEnd<(usize, usize)>) -> Result<(), nix::Error> {

    let resize_action = signal::SigAction::new(
        handle_resize, signal::SockFlag::empty(), signal::SigSet::empty());

    unsafe {
        try!(signal::sigaction(signal::SIGWINCH, &resize_action))
    };

    mioco::spawn(move || {
        loop {
            if RESIZED.compare_and_swap(true, false, atomic::Ordering::SeqCst) {
                let mut ws: ffi::WinSize;
                unsafe {
                    ws = mem::uninitialized();
                    nix::from_ffi(ffi::ioctl(tty_fd, ffi::TIOCGWINSZ, &mut ws)).unwrap();
                }

                size_mailbox.send((ws.ws_col as usize, ws.ws_row as usize));
            }
            mioco::sleep(10);
        }
    });

    Ok(())
}

extern fn handle_resize(_: libc::c_int) {
    RESIZED.store(true, atomic::Ordering::SeqCst);
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
