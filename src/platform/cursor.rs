use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::rust_connection::RustConnection;

pub fn position() -> Option<(i16, i16)> {
    let (conn, screen_num) = RustConnection::connect(None).ok()?;

    let screen = &conn.setup().roots[screen_num];

    let reply = conn
        .query_pointer(screen.root)
        .ok()?
        .reply()
        .ok()?;

    Some((reply.root_x, reply.root_y))
}
