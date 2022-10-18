use utils::GadgetFactory;

pub static GADGET_FACTORIES: &[&(dyn GadgetFactory + Sync)] = &[
    #[cfg(feature = "clock")]
    &clock::ClockGadgetFactory,
    #[cfg(feature = "weather")]
    &weather::WeatherGadgetFactory,
    #[cfg(feature = "cses-status")]
    &cses_status::CSESStatusGadgetFactory,
    #[cfg(feature = "safebooru-waifu")]
    &safebooru_waifu::WaifuGadgetFactory,
];
