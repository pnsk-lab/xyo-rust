use std::io::{self, IsTerminal, Read};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

#[cfg(unix)]
use termios::{ECHO, ICANON, TCSANOW, Termios, VMIN, VTIME, tcsetattr};

#[cfg(unix)]
struct RawModeGuard {
    fd: i32,
    original: Termios,
}

#[cfg(unix)]
impl RawModeGuard {
    fn enable(fd: i32) -> io::Result<Self> {
        let original =
            Termios::from_fd(fd).map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;
        let mut raw = original.clone();
        raw.c_lflag &= !(ICANON | ECHO);
        raw.c_cc[VMIN] = 1;
        raw.c_cc[VTIME] = 0;
        tcsetattr(fd, TCSANOW, &raw)
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;
        Ok(Self { fd, original })
    }
}

#[cfg(unix)]
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = tcsetattr(self.fd, TCSANOW, &self.original);
    }
}

/// Spawn a background thread watching stdin for `Esc` (ASCII 27) and flip
/// `stop_requested` once seen. Returns the thread handle so the caller can
/// join later; the thread switches stdin into raw mode while running.
pub fn spawn_escape_listener(
    stop_requested: Arc<std::sync::atomic::AtomicBool>,
) -> io::Result<Option<thread::JoinHandle<()>>> {
    #[cfg(not(unix))]
    {
        let _ = stop_requested;
        Ok(None)
    }

    #[cfg(unix)]
    {
        if !io::stdin().is_terminal() {
            return Ok(None);
        }

        let handle = thread::spawn(move || {
            let stdin = io::stdin();
            let fd = stdin.as_raw_fd();
            let _guard = RawModeGuard::enable(fd).ok();
            let mut buffer = [0u8; 1];
            let mut reader = stdin.lock();
            while !stop_requested.load(Ordering::Relaxed) {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(_) if buffer[0] == 27 => {
                        stop_requested.store(true, Ordering::Relaxed);
                        break;
                    }
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
        });
        Ok(Some(handle))
    }
}
