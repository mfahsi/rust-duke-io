enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

pub fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size / 1_000_000_000),
        _ => FileSize::Terabytes(size / 1_000_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64),
    }
}

pub fn dispay_sizes(bytes: u64){
    let kb = bytes / 1000;
    let mb = kb / 1000;
    let gb = mb / 1000;
    print!("{}B,{}KB,{}MB,{}GB",bytes,kb,mb,gb);

}
pub fn to_bytes(size: u64, unit:&str) -> u64 {
    match unit {
        "bytes" => size,
        "kb" => size * 1000,
        "mb" => size * 1000_000,
        "gb" => size * 1000_000_000,
        _ => panic!("{} is not a valid unit", unit),

    }
}