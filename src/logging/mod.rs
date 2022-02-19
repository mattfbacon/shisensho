use crate::config::{LoggingConfig, LoggingSink};
use log4rs::append::{console::ConsoleAppender, file::FileAppender};
use log4rs::config::Appender;

pub fn init(config: &[LoggingConfig]) -> anyhow::Result<log4rs::Handle> {
	let (setup, names) = config.iter().fold(Ok((log4rs::config::Config::builder(), Vec::new())), move |acc: anyhow::Result<_>, sink| {
		let (setup, mut names) = acc?;
		let (name, appender, level) = appender_for_sink(sink)?;
		let filter = Box::new(log4rs::filter::threshold::ThresholdFilter::new(level));
		let appender = Appender::builder().filter(filter).build(name.clone(), appender);
		names.push(name);
		Ok((setup.appender(appender), names))
	})?;
	Ok(log4rs::init_config(
		setup.build(names.into_iter().fold(log4rs::config::runtime::Root::builder(), |builder, name| builder.appender(name)).build(log::LevelFilter::max()))?,
	)?)
}

fn appender_for_sink(sink: &LoggingConfig) -> anyhow::Result<(String, Box<dyn log4rs::append::Append>, log::LevelFilter)> {
	let (name, appender): (String, Box<dyn log4rs::append::Append>) = match &sink.sink {
		LoggingSink::File { file } => (format!("file:{}", file.display()), Box::new(FileAppender::builder().build(file)?)),
		LoggingSink::Console { console } => (format!("console:{:?}", console), Box::new(ConsoleAppender::builder().target((*console).into()).build())),
	};
	Ok((name, appender, sink.level))
}
