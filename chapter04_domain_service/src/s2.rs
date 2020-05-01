pub struct PhysicalDistributionBase {}

pub struct Baggage {}

pub struct TransportService {}

impl PhysicalDistributionBase {
    pub fn ship(&self, baggage: Baggage) -> Baggage {
    }

    pub fn receive(&self, baggage: Baggage) {
    }
}

impl TransportService {
    pub fn transport(&self, from: PhysicalDistributionBase, to: PhysicalDistributionBase, baggage: Baggage) {
        let shipped_baggage = from.ship(baggage);
        to.receive(shipped_baggage);

        // record transport
    }
}