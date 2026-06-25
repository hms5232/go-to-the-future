use std::env::args;

fn main() {
    let target; // Target time to arrive

    // Parse target time argument
    match args().skip(1).next() {
        Some(arg) => match humantime::parse_duration(arg.as_str()) {
            Ok(d) => target = d,
            Err(e) => {
                eprintln!("Invalid target time: {}", e);
                return;
            }
        },
        None => {
            eprintln!("Target time is required.");
            return;
        }
    }

    let bar = indicatif::ProgressBar::new(target.as_millis() as u64);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner} {percent}% {wide_bar:.cyan/blue}")
            .unwrap()
            .tick_chars("🕛🕧🕐🕜🕑🕝🕒🕞🕓🕟🕔🕠🕕🕡🕖🕢🕗🕣🕘🕤🕙🕥🕚🕦"),
    );
    let mut last = bar.elapsed();

    while bar.elapsed() < target {
        // Increment progress bar by elapsed time
        let current = bar.elapsed();
        bar.inc(current.abs_diff(last).as_millis() as u64);
        last = current;
        // Wait for 16ms to match 60 FPS, etc.
        std::thread::sleep(std::time::Duration::from_millis(32));
    }
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("⌛ {percent}% {wide_bar:.cyan/blue} {msg}")
            .unwrap(),
    );
    bar.finish_with_message("🎉 Arrived");
}
