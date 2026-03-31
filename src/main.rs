use std::{
    fs::{self, Metadata},
    path::Path,
};

use chrono::{DateTime, Utc};

fn main() {
    let metadata: Metadata = fs::metadata(Path::new("/")).expect("Failed to get metadata of '/'");
    let created: DateTime<Utc> = metadata
        .created()
        .expect("Failed to get date of birth")
        .into();

    println!(
        "System was installed on {}",
        created.format("%Y-%m-%d, %H:%M:%S")
    );
}
