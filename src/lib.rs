use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use image::ImageFormat;
pub use vtf_wrapper::VtfWrapper;

mod vtf_wrapper;

/// Converts a file or a directory to the specified format
pub fn convert<P>(input: P, output: P, format: &Option<String>) -> Result<(), String>
where
    P: AsRef<Path>,
{
    if input.as_ref().is_dir() && output.as_ref().is_dir() {
        let format = match format {
            Some(f) => f,
            None => return Err("You must specify a format".to_string()),
        };

        return convert_directory(input, output, format);
    }

    return convert_file(input, output, format);
}

/// Converts a directory, files it can't convert will be ignored.
/// The output directory will have the same structure as the input directory.
pub fn convert_directory<P>(input: P, output: P, format: &String) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let input_path = input.as_ref();
    let output_path = output.as_ref();

    // Recursively iterate over all files in the input directory and its subdirectories
    for entry in walkdir::WalkDir::new(input_path).follow_links(true) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return Err(e.to_string()),
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let input_file_path = entry.path();

        // Construct the corresponding output file path with the same directory structure
        let input_file_relpath = input_file_path
            .strip_prefix(input_path)
            .map_err(|e| e.to_string())?;

        let mut output_file_path = output_path.join(input_file_relpath);
        output_file_path.set_extension(format);

        // Ensure the output directory exists
        if let Some(output_file_parent) = output_file_path.parent() {
            if !output_file_parent.exists() {
                create_dir_all(output_file_parent).map_err(|e| e.to_string())?;
            }
        }

        // Convert the file and save it to the output directory
        _ = convert_file(
            &input_file_path,
            &output_file_path.as_path(),
            &Some(format.clone()),
        );
    }

    Ok(())
}

/// Converts a file from one format to the other. If no format is given,
/// tries to determine the format by file extension.
pub fn convert_file<P>(input: P, output: P, format: &Option<String>) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let wrapper = VtfWrapper::from_file(input)?;
    let format = match format {
        Some(f) => f,
        None => {
            let extension = match output.as_ref().extension() {
                Some(ext) => ext,
                None => return Err("No file extension was given".to_string()),
            };

            extension
                .to_str()
                .ok_or("No file extension was found".to_string())?
        }
    };

    if format == "vtf" {
        let data = wrapper.to_vtf()?;
        File::create(output)
            .map_err(|e| e.to_string())?
            .write_all(&data)
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let format = get_format(format)?;
    wrapper
        .image
        .save_with_format(output, format)
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Gets the image format from a string
fn get_format(format: &str) -> Result<ImageFormat, String> {
    let format = match format.to_lowercase().as_str() {
        "png" => ImageFormat::Png,
        "jpg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::WebP,
        "avif" => ImageFormat::Avif,
        "bmp" => ImageFormat::Bmp,
        "ico" => ImageFormat::Ico,
        "tga" => ImageFormat::Tga,
        "gif" => ImageFormat::Gif,
        "pnm" => ImageFormat::Pnm,
        "tiff" => ImageFormat::Tiff,
        "hdr" => ImageFormat::Hdr,
        "ff" => ImageFormat::Farbfeld,
        _ => return Err(format!("Unsupported format: {}", format)),
    };

    Ok(format)
}
