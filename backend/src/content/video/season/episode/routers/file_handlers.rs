use tokio::process::Command;
use std::path::Path;

pub async fn convert_to_mp4(input_path: &str, output_path: &str) -> Result<(), String>{
    if !Path::new(input_path).exists(){
        return Err(format!("Input file not found: {}", input_path));
    }

     let status = Command::new("ffmpeg")
        .args(&[
            "-i", input_path,
            "-c:v", "libx264", // H.264 video
            "-c:a", "aac",     // AAC audio
            "-y",              // overwrite without asking
            output_path,
        ])
        .status()
        .await
        .map_err(|e| format!("Failed to run ffmpeg: {}", e))?;

    if !status.success() {
        return Err(format!("ffmpeg failed with exit code {:?}", status.code()));
    }

    Ok(())
}