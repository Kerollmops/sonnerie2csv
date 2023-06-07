use std::{env, io};

use sonnerie::{DatabaseReader, Wildcard};

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("missing database path");
    let reader = DatabaseReader::new(path.as_ref())?;
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["stream", "timestamp", "username", "message"])?;

    let wildcard = Wildcard::new("%");
    for record in reader.get_filter(&wildcard) {
        writer.write_record(&[
            record.key(),
            &record.timestamp_nanos().to_string(),
            record.get(0),
            record.get(1),
        ])?;
    }

    Ok(())
}
