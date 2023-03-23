pub fn convert_file_by_ffmpeg(
    input_file_path: &str,
    output_file_path: &str,
    sample_rate: u32,
) -> i32 {
    let mut command = std::process::Command::new("ffmpeg");
    command
        .arg("-y")
        .arg("-i")
        .arg(input_file_path)
        .arg("-vn")
        .arg("-ar")
        .arg(format!("{}", sample_rate))
        .arg("-ac")
        .arg("1")
        .arg("-b:a")
        .arg("192k")
        .arg(output_file_path);
    // print command
    // println!("command: {:?}", command);
    match command.status() {
        Ok(status) => {
            println!("convert command exit status: {}", status);
            status.code().unwrap()
        }
        Err(err) => {
            println!("convert process error: {:?}", err);
            1
        }
    }
}

pub fn split_file_by_ffmpeg(input_file_path: &str, output_file_path: &str) -> i32 {
    let mut command = std::process::Command::new("ffmpeg");
    command
        .arg("-y")
        .arg("-i")
        .arg(input_file_path)
        .arg("-f")
        .arg("segment")
        .arg("-segment_time")
        .arg("10")
        .arg("-c")
        .arg("copy")
        .arg(output_file_path);
    // print command
    // println!("command: {:?}", command);
    match command.status() {
        Ok(status) => {
            println!("split command exit status: {}", status);
            status.code().unwrap()
        }
        Err(err) => {
            println!("split process error: {:?}", err);
            1
        }
    }
}
