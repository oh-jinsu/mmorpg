use std::error::Error;

use crate::{world::World, net::packet, common::math::Vector3};

///
/// Handle the request for ping.
/// 
/// Just return the passed timestamp to the connection.
/// 
pub fn handle(timestamp: i64, current: Vector3, context: &mut World) -> Result<(), Box<dyn Error>> {
    let conn = match context.connections.get(&current) {
        Some(conn) => conn,
        None => return Ok(())
    };

    let outgoing = packet::Outgoing::Pong { timestamp };

    conn.try_write_one(&mut outgoing.serialize())?;

    Ok(())
}
