use std::{ffi::OsStr, fs::File, io::Read, path::Path};

use image::{io::Reader as ImageReader, DynamicImage, ImageBuffer};
use vtf::vtf::VTF;
use vtflib::{VtfFile, VtfLib};

/// Wraps a dynamic image to provide it with Valve's VTF file conversion abilities
pub struct VtfWrapper {
    pub image: DynamicImage,
}

impl VtfWrapper {
    /// Creates a new instance with a dynamic image
    pub fn new(image: DynamicImage) -> Self {
        Self { image }
    }

    /// Determines the image type and attempts to convert it (works with VTF files)
    pub fn from_file<P>(path: P) -> Result<Self, String>
    where
        P: AsRef<Path>,
    {
        let extension = path.as_ref().extension();

        if extension == Some(OsStr::new("vtf")) {
            let bytes = bytes_from_path(path)?;
            return Self::from_vtf(&bytes);
        }

        return Self::from_image(path);
    }

    /// Converts the dynamic image to VTF data
    pub fn to_vtf(&self) -> Result<Vec<u8>, String> {
        let image = VTF::create(self.image.clone(), vtf::ImageFormat::Rgba8888)
            .map_err(|e| e.to_string())?;

        return Ok(image);
    }

    /// Constructs the wrapper from VTF data
    pub fn from_vtf(bytes: &[u8]) -> Result<Self, String> {
        let (lib, mut guard) =
            VtfLib::initialize().ok_or("Could not initialize VtfLib".to_string())?;

        let file = lib.new_vtf_file();
        let mut file_bound = file.bind(&mut guard);

        file_bound.load(&bytes).map_err(|e| e.to_string())?;

        let buf = VtfFile::convert_image_to_rgba8888(
            bytes,
            file_bound.width(),
            file_bound.height(),
            vtflib::ImageFormat::Rgba8888,
        )
        .map_err(|e| e.to_string())?;

        let buffer = match ImageBuffer::from_raw(file_bound.width(), file_bound.height(), buf) {
            Some(buf) => buf,
            None => return Err("Could not get image buffer".to_string()),
        };

        Ok(Self {
            image: DynamicImage::ImageRgba8(buffer),
        })
    }

    fn from_image<P>(path: P) -> Result<Self, String>
    where
        P: AsRef<Path>,
    {
        let image_enc = ImageReader::open(path).map_err(|e| e.to_string())?;
        let image = image_enc.decode().map_err(|e| e.to_string())?;

        return Ok(Self { image });
    }
}

fn bytes_from_path<P>(path: P) -> Result<Vec<u8>, String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path).map_err(|e| e.to_string())?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    Ok(bytes)
}
