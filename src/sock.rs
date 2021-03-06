use libc::close;

use nix::Result;
use nix::sys::socket::{
    AddressFamily,
    bind,
    getsockname,
    InetAddr,
    listen,
    socket,
    SockAddr,
    SockFlag,
    SockType,
};

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

pub fn on(addr: SocketAddr) -> Result<RawFd> {
    let sock = try!(socket(
		AddressFamily::Inet,
		SockType::Stream,
		SockFlag::empty(),
		0 // protocol
	));

    let result = Ok(())
		.and_then(|_| bind(sock, &SockAddr::new_inet(
			InetAddr::from_std(&addr))
		))
    	.and_then(|_| listen(sock, 1))
		.and_then(|_| Ok(sock));

    if !result.is_ok() {
        unsafe { close(sock) };
    }

	result
}

pub fn at(sock: RawFd) -> Result<SockAddr> {
    getsockname(sock)
}
