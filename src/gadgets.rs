use utils::GadgetFactory;

pub const GADGET_FACTORIES: &[&dyn GadgetFactory] = &[
    #[cfg(feature = "clock")]
    &clock::ClockGadgetFactory,
    #[cfg(feature = "weather")]
    &weather::WeatherGadgetFactory,
    #[cfg(feature = "cses-status")]
    &cses_status::CSESStatusGadgetFactory,
];
