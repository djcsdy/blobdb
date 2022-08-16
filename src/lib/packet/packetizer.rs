use crate::lib::packet::Packet;

pub trait Packetizer<FinalizeData, Finalizer: PacketizerFinalizer<FinalizeData>> {
    fn next_packet(&mut self, max_size: u16) -> Packetized<FinalizeData>;
    fn into_finalizer(self) -> Finalizer;
}

pub trait PacketizerFinalizer<U> {
    fn finalize(&mut self, packet: Packet, finalize_data: U) -> Packet;
}

pub enum Packetized<U> {
    Packet { packet: Packet, finalize_data: U },
    PacketTooBig,
    End,
}
