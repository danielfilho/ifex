import piexif from 'piexifjs';

/**
 * EXIF tag definitions and mappings
 */
export const EXIF_TAGS = {
  cameraMaker: ['0th', piexif.ImageIFD.Make],
  cameraModel: ['0th', piexif.ImageIFD.Model],
  lensMaker: ['Exif', piexif.ExifIFD.LensMake],
  lensModel: ['Exif', piexif.ExifIFD.LensModel],
  focalLength: ['Exif', piexif.ExifIFD.FocalLength],
  fNumber: ['Exif', piexif.ExifIFD.FNumber],
  filmISO: ['Exif', piexif.ExifIFD.ISOSpeedRatings],
  shotISO: ['Exif', piexif.ExifIFD.ISOSpeed],
  photographer: ['0th', piexif.ImageIFD.Artist]
};

/**
 * Set an EXIF value in the EXIF object
 */
export function setExifValue(exifObj, configKey, value) {
  if (!value || (typeof value === 'string' && value.trim() === '')) return;

  const [ifd, tag] = EXIF_TAGS[configKey];
  if (!exifObj[ifd]) {
    exifObj[ifd] = {};
  }
  exifObj[ifd][tag] = value;
}

/**
 * Create a clean EXIF object structure
 */
export function createCleanExifObject() {
  return {
    '0th': {},
    'Exif': {},
    'GPS': {},
    'Interop': {},
    '1st': {},
    'thumbnail': null
  };
}
