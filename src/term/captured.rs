use nix;
use nix::sys::termios;

use std::os::unix;

pub struct CapturedTerm {
    tty_fd: unix::io::RawFd,
    orig_tios: termios::Termios,
}

impl CapturedTerm {
    pub fn create(tty_fd: unix::io::RawFd) -> Result<Self, nix::Error> {
        let mut tios = try!(termios::tcgetattr(tty_fd));
        let orig_tios = tios.clone();
        config_tios(&mut tios);
        try!(termios::tcsetattr(tty_fd, termios::SetArg::TCSAFLUSH, &tios));

        Ok(CapturedTerm {
            tty_fd: tty_fd,
            orig_tios: orig_tios,
        })
    }
}

impl Drop for CapturedTerm {
    fn drop(&mut self) {
        termios::tcsetattr(
            self.tty_fd, termios::SetArg::TCSAFLUSH, &self.orig_tios).unwrap();
    }
}

fn config_tios(tios: &mut termios::Termios) {
    use nix::sys::termios::*;

    tios.c_iflag = tios.c_iflag & !(IGNBRK | BRKINT | PARMRK | ISTRIP |
                                    INLCR | IGNCR | ICRNL | IXON);
    tios.c_oflag = tios.c_oflag & !OPOST;
    tios.c_lflag = tios.c_lflag & !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    tios.c_cflag = tios.c_cflag & !(CSIZE | PARENB);
    tios.c_cflag = tios.c_cflag | CS8;
    tios.c_cc[VMIN] = 0;
    tios.c_cc[VTIME] = 0;
}
