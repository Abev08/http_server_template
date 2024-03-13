use std::{io::Write, thread, time::Duration};

use chrono::Local;

mod server;

fn main() {
  // logger setup
  env_logger::Builder::new()
    .format(|buf, record| {
      let style = buf.default_level_style(record.level());
      return writeln!(
        buf,
        "[{} {}{}{}] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        style.render(),
        record.level(),
        style.render_reset(),
        record.args(),
      );
    })
    .filter_level(log::LevelFilter::Info)
    .init();

  // start http server
  server::start("127.0.0.1");

  // do nothing
  let sleep_dur = Duration::from_millis(1000);
  loop {
    thread::sleep(sleep_dur);
  }
}
