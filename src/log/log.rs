use std::time::SystemTime;

pub fn setup_logger() -> anyhow::Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Off)
        .filter(|metadata| metadata.target().starts_with("procvamp"))
        .level_for("procvamp", log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;

    // Set a panic hook so we can log errors too
    std::panic::set_hook(Box::new(|panic_info| {
        log::error!("{panic_info}");
    }));
    Ok(())
}
