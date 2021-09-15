use crate::lib::packet::Packet;

pub trait Packetizer<PostUpdate, PostUpdater: PacketizerPostUpdater<PostUpdate>> {
    fn next_packet(&mut self, max_size: u16) -> Packetized<PostUpdate>;
    fn into_post_updater(self) -> PostUpdater;
}

pub trait PacketizerPostUpdater<U> {
    fn apply_post_update(&mut self, packet: &Packet, post_update: U) -> Packet;
}

pub enum Packetized<U> {
    Packet { packet: Packet, post_update: U },
    PacketTooBig,
    End,
}
