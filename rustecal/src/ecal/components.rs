use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct EcalComponents: u32 {
        const NONE       = 0x000;
        const PUBLISHER  = 0x001;
        const SUBSCRIBER = 0x002;
        const SERVICE    = 0x004;
        const MONITORING = 0x008;
        const LOGGING    = 0x010;
        const TIMESYNC   = 0x020;

        const DEFAULT = Self::PUBLISHER.bits()
                      | Self::SUBSCRIBER.bits()
                      | Self::SERVICE.bits()
                      | Self::LOGGING.bits()
                      | Self::TIMESYNC.bits();

        const ALL     = Self::PUBLISHER.bits()
                      | Self::SUBSCRIBER.bits()
                      | Self::SERVICE.bits()
                      | Self::MONITORING.bits()
                      | Self::LOGGING.bits()
                      | Self::TIMESYNC.bits();
    }
}
