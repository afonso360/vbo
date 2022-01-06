use std::io::Write;
use crate::{Result, Error, Channel, ChannelValue};
use time::{format_description, OffsetDateTime};

pub struct Writer {
    file_creation_time: Option<OffsetDateTime>,
    comment: Option<String>,
    channels: Vec<Channel>,
    samples: Vec<Vec<ChannelValue>>,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            file_creation_time: None,
            comment: None,
            channels: Vec::new(),
            samples: Vec::new(),
        }
    }

    pub fn set_file_creation_time(&mut self, time: OffsetDateTime) {
        self.file_creation_time = Some(time);
    }

    pub fn set_comment(&mut self, comment: String) {
        self.comment = Some(comment);
    }

    pub fn add_channel(&mut self, new: Channel) -> Result<()> {
        if self.channels.iter().find(|c| c.name == new.name).is_some() {
            return Err(Error::DuplicateChannel(new.name.clone()));
        }

        self.channels.push(new);
        Ok(())
    }

    pub fn add_samples(&mut self, line: Vec<ChannelValue>) {
        self.samples.push(line);
    }


    pub fn write_to<W: Write>(&self, sink: &mut W) -> Result<()> {
        // Write File comment
        let date_time = self.file_creation_time.unwrap_or_else(|| OffsetDateTime::now_utc());
        let date = {
            let format = format_description::parse("[day padding:zero]/[month padding:zero repr:numerical]/[year repr:full padding:zero]").unwrap();
            date_time.date().format(&format)?
        };
        let time = {
            let format = format_description::parse("[hour padding:zero]:[minute padding:zero]:[second padding:zero]").unwrap();
            date_time.time().format(&format)?
        };
        write!(sink, "File created on {} at {}\n\n", date, time)?;

        // Write Headers
        writeln!(sink, "[header]")?;
        for channel in &self.channels {
            writeln!(sink, "{}", channel)?;
        }
        write!(sink, "\n")?;

        // Write Comment
        if let Some(comment) = &self.comment {
            writeln!(sink, "[comments]\n{}\n", comment)?;
        }

        // Write Column Names
        // TODO: It looks like channel names and column names differ
        writeln!(sink, "[column names]")?;
        for channel in &self.channels {
            // TODO: This leaves a trailing whitespace...
            write!(sink, "{} ", channel.name)?;
        }
        write!(sink, "\n\n")?;

        // Write Data
        writeln!(sink, "[data]")?;
        for sample_line in &self.samples {
            for sample in sample_line {
                    // TODO: This leaves a trailing whitespace...
                write!(sink, "{} ", sample)?;
            }
            write!(sink, "\n")?;
        }
        write!(sink, "\n")?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{ChannelName, ChannelUnit};
    use super::*;

    fn writer_contains(w: &Writer, s: &str) -> bool {
        let mut file = vec![0; 2048];
        {
            let mut cursor = Cursor::new(&mut file);
            w.write_to(&mut cursor).unwrap();
        }

        let str = std::str::from_utf8(&file[..]).unwrap();
        println!("str: {}", str);
        str.contains(s)
    }

    #[test]
    fn write_date_time() {
        let dt = OffsetDateTime::from_unix_timestamp(1641469669).unwrap();
        let mut writer = Writer::new();
        writer.set_file_creation_time(dt);

        assert!(writer_contains(&writer, "File created on 06/01/2022 at 11:47:49\n\n"));
    }

    #[test]
    fn writes_headers() {
        let mut writer = Writer::new();
        writer.add_channel(Channel{
            name: ChannelName::Satellites,
            unit: None
        }).unwrap();
        writer.add_channel(Channel{
            name: ChannelName::Custom("Hello".into()),
            unit: Some(ChannelUnit::G),
        }).unwrap();

        assert!(writer_contains(&writer, "\n[header]\nsatellites\nHello g\n"));
    }


    #[test]
    fn writes_comment() {
        let mut writer = Writer::new();
        writer.set_comment(String::from("Cool Comment\nWith a newline"));

        assert!(writer_contains(&writer, "\n[comments]\nCool Comment\nWith a newline\n"));
    }


    #[test]
    fn writes_column_names() {
        let mut writer = Writer::new();
        writer.add_channel(Channel{
            name: ChannelName::Satellites,
            unit: None
        }).unwrap();
        writer.add_channel(Channel{
            name: ChannelName::Custom("Hello".into()),
            unit: Some(ChannelUnit::G),
        }).unwrap();

        assert!(writer_contains(&writer, "\n[column names]\nsatellites Hello \n"));
    }

    #[test]
    fn writes_data() {
        let mut writer = Writer::new();
        writer.add_channel(Channel{
            name: ChannelName::Satellites,
            unit: None
        }).unwrap();
        writer.add_channel(Channel{
            name: ChannelName::Custom("Hello".into()),
            unit: Some(ChannelUnit::G),
        }).unwrap();

        writer.add_samples(vec![
            ChannelValue::Satellites(0),
            ChannelValue::Velocity(10.0),
        ]);
        writer.add_samples(vec![
            ChannelValue::Satellites(23),
            ChannelValue::Velocity(300.0),
        ]);

        assert!(writer_contains(&writer, "\n[data]\n"));
        assert!(writer_contains(&writer, "\n000 010.000 \n"));
        assert!(writer_contains(&writer, "\n023 300.000 \n"));
    }
}
