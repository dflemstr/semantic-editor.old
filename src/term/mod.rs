use mio;
use mioco;

use nix;
use nix::fcntl;

use std::fs;
use std::io;
use std::sync::atomic;

mod captured;
mod resize;

static TTY_LOCK: atomic::AtomicBool = atomic::ATOMIC_BOOL_INIT;

pub struct Term {
    _capture: captured::CapturedTerm,
    _tty_file: fs::File,
    tty: mioco::EventSource<mio::Io>,
    pub events_recv: mioco::EventSource<mioco::MailboxInnerEnd<Event>>,
}

#[derive(Debug)]
pub enum Event {
    Resized(usize, usize),
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Nix(nix::Error),
    TtyLockTaken,
}

impl Term {
    pub fn new(mioco: &mut mioco::MiocoHandle) -> Result<Self, Error> {
        use std::os::unix::io::AsRawFd;
        use std::io::{Read,Write};

        if TTY_LOCK.compare_and_swap(false, true, atomic::Ordering::SeqCst) {
            return Err(Error::TtyLockTaken)
        }

        let tty_file = try!(
            fs::OpenOptions::new()
                .write(true)
                .read(true)
                .open("/dev/tty"));

        let tty_fd = tty_file.as_raw_fd();
        try!(fcntl::fcntl(tty_fd, fcntl::FcntlArg::F_SETFL(fcntl::O_NONBLOCK)));
        let tty = mioco.wrap(mio::Io::from_raw_fd(tty_fd));

        let capture = try!(captured::CapturedTerm::create(tty_fd));

        let (events_send, events_recv) = mioco::mailbox();
        let (resize_send, resize_recv) = mioco::mailbox();

        // Unsafe because TTY_LOCK must be held
        unsafe {
            try!(resize::send_resizes_to(tty_fd, resize_send))
        };

        let term = Term {
            _capture: capture,
            _tty_file: tty_file,
            tty: tty,
            events_recv: mioco.wrap(events_recv),
        };

        mioco.spawn(move |mioco| {
            let resize_recv = mioco.wrap(resize_recv);
            loop {
                let r = resize_recv.read();
                events_send.send(Event::Resized(r.0, r.1));
            }
        });

        Ok(term)
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        TTY_LOCK.store(false, atomic::Ordering::SeqCst);
    }
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
