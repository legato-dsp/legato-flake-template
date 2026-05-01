use legato::{
    builder::{LegatoBuilder, Unconfigured},
    config::Config,
    interface::AudioInterface,
    ports::PortBuilder,
};

fn main() {
    let graph = std::fs::read_to_string("../.legato").unwrap();

    let config = Config {
        sample_rate: 44_100,
        block_size: 4096,
        channels: 2,
        rt_capacity: 0,
    };

    let ports = PortBuilder::default().audio_out(2).build();

    let (app, _) = LegatoBuilder::<Unconfigured>::new(config, ports).build_dsl(&graph);

    #[cfg(target_os = "macos")]
    let host = cpal::host_from_id(cpal::HostId::CoreAudio).expect("JACK host not available");

    #[cfg(target_os = "linux")]
    let host = cpal::host_from_id(cpal::HostId::Jack).expect("JACK host not available");

    AudioInterface::builder(&host, config)
        .build(app)
        .expect("Failed to start audio")
        .run_forever();
}
