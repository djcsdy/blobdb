use crate::lib::packet::Packet;

pub trait Packetizer<U> {
    fn next_packet(&mut self, max_size: u16) -> Packetized<U>;
    fn apply_post_update(&mut self, packet: &mut Packet, post_update: U);
}

pub enum Packetized<U> {
    Packet { packet: Packet, post_update: U },
    PacketTooBig,
    End,
}
