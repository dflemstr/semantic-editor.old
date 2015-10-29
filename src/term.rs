use mio;
use mioco;

use nix;
use nix::sys::termios;
use nix::fcntl;

use std::fs;
use std::io;
use std::os::unix;

pub struct Term {
    orig_tios: termios::Termios,
    tty_file: fs::File,
    tty: mioco::EventSource<mio::Io>,
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Nix(nix::Error),
}

impl Term {
    pub fn new(mioco: &mut mioco::MiocoHandle) -> Result<Self, Error> {
        use std::os::unix::io::{AsRawFd,FromRawFd};
        use std::io::{Read,Write};

        let tty_file = try!(
            fs::OpenOptions::new()
                .write(true)
                .read(true)
                .open("/dev/tty"));

        let tty_fd = tty_file.as_raw_fd();
        try!(fcntl::fcntl(tty_fd, fcntl::FcntlArg::F_SETFL(fcntl::O_NONBLOCK)));
        let tty = mioco.wrap(mio::Io::from_raw_fd(tty_fd));

        let orig_tios = try!(push_term_tios(tty_fd));

        Ok(Term {
            orig_tios: orig_tios,
            tty_file: tty_file,
            tty: tty,
        })
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        use std::os::unix::io::AsRawFd;
        pop_term_tios(self.tty_file.as_raw_fd(), &self.orig_tios).unwrap();
    }
}

fn push_term_tios(tty_fd: unix::io::RawFd)
                  -> Result<termios::Termios, nix::Error> {

    let mut tios = try!(termios::tcgetattr(tty_fd));
    let orig_tios = tios.clone();
    config_tios(&mut tios);
    try!(termios::tcsetattr(tty_fd, termios::SetArg::TCSAFLUSH, &tios));

    Ok(orig_tios)
}

fn pop_term_tios(tty_fd: unix::io::RawFd, termios: &termios::Termios)
                 -> Result<(), nix::Error> {
    termios::tcsetattr(tty_fd, termios::SetArg::TCSAFLUSH, termios)
}

fn config_tios(tios: &mut termios::Termios) {
    use nix::sys::termios::{
        IGNBRK, BRKINT, PARMRK, ISTRIP, INLCR, IGNCR, ICRNL, IXON, OPOST, ECHO,
        ECHONL, ICANON, ISIG, IEXTEN, CSIZE, PARENB, CS8, VMIN, VTIME
    };

    tios.c_iflag = tios.c_iflag & !(IGNBRK | BRKINT | PARMRK | ISTRIP |
                                    INLCR | IGNCR | ICRNL | IXON);
    tios.c_oflag = tios.c_oflag & !OPOST;
    tios.c_lflag = tios.c_lflag & !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    tios.c_cflag = tios.c_cflag & !(CSIZE | PARENB);
    tios.c_cflag = tios.c_cflag | CS8;
    tios.c_cc[VMIN] = 0;
    tios.c_cc[VTIME] = 0;
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<nix::Error> for Error {
    fn from(e: nix::Error) -> Self {
        Error::Nix(e)
    }
}
