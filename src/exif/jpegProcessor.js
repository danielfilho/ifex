import fs from 'fs';
import piexif from 'piexifjs';
import { setExifValue, createCleanExifObject } from './tags.js';

/**
 * JPEG EXIF processing
 */
export class JpegProcessor {
  async applyExifToFile(filePath, selection) {
    try {
      this.validateSelection(selection);

      const data = fs.readFileSync(filePath);
      const jpegData = data.toString('binary');

      let exifObj;
      try {
        exifObj = piexif.load(jpegData);
      } catch {
        exifObj = createCleanExifObject();
      }

      this.setAllExifValues(exifObj, selection);

      let exifBytes;
      try {
        exifBytes = piexif.dump(exifObj);
      } catch (dumpError) {
        console.warn(`EXIF dump failed, retrying with basic tags: ${dumpError.message}`);
        exifBytes = this.createBasicExif(selection);
      }

      const newJpegData = piexif.insert(exifBytes, jpegData);
      const newBuffer = Buffer.from(newJpegData, 'binary');
      fs.writeFileSync(filePath, newBuffer);

      return true;
    } catch (error) {
      console.error(`Error processing JPEG ${filePath}:`, error.message);
      return false;
    }
  }

  validateSelection(selection) {
    if (!selection) {
      throw new Error('Selection object is required');
    }
    if (!selection.setup || !selection.setup.camera || !selection.setup.lens) {
      throw new Error('Selection must include setup with camera and lens');
    }
    if (!selection.film) {
      throw new Error('Selection must include film information');
    }
  }

  setAllExifValues(exifObj, selection) {
    // Camera information
    setExifValue(exifObj, 'cameraMaker', selection.setup.camera.maker);
    setExifValue(exifObj, 'cameraModel', selection.setup.camera.model);
    setExifValue(exifObj, 'photographer', selection.photographer);

    // ISO information
    setExifValue(exifObj, 'filmISO', [selection.film.iso]);
    if (selection.shotISO && selection.shotISO !== selection.film.iso) {
      setExifValue(exifObj, 'shotISO', selection.shotISO);
    }

    // Lens information
    setExifValue(exifObj, 'lensMaker', selection.setup.lens.maker);
    setExifValue(exifObj, 'lensModel', selection.setup.lens.getLensModelWithAperture());

    // Focal length and aperture as separate numeric values
    if (selection.setup.lens.focalLength) {
      const focalLength = parseInt(selection.setup.lens.focalLength);
      setExifValue(exifObj, 'focalLength', [focalLength, 1]);
    }

    if (selection.setup.lens.aperture) {
      const aperture = parseFloat(selection.setup.lens.aperture);
      setExifValue(exifObj, 'fNumber', [Math.round(aperture * 10), 10]);
    }
  }

  createBasicExif(selection) {
    const basicExifObj = createCleanExifObject();

    // Only set basic tags that are known to work
    if (selection.setup.camera.maker) {
      basicExifObj['0th'][piexif.ImageIFD.Make] = selection.setup.camera.maker;
    }
    if (selection.setup.camera.model) {
      basicExifObj['0th'][piexif.ImageIFD.Model] = selection.setup.camera.model;
    }
    if (selection.photographer) {
      basicExifObj['0th'][piexif.ImageIFD.Artist] = selection.photographer;
    }
    if (selection.film.iso) {
      basicExifObj['Exif'][piexif.ExifIFD.ISOSpeedRatings] = [selection.film.iso];
    }
    if (selection.shotISO && selection.shotISO !== selection.film.iso) {
      basicExifObj['Exif'][piexif.ExifIFD.ISOSpeed] = selection.shotISO;
    }

    try {
      return piexif.dump(basicExifObj);
    } catch (secondError) {
      console.warn(`Basic EXIF dump also failed, using minimal tags: ${secondError.message}`);
      return this.createMinimalExif(selection);
    }
  }

  createMinimalExif(selection) {
    const minimalExifObj = {
      '0th': {
        [piexif.ImageIFD.Make]: selection.setup.camera.maker || 'Unknown',
        [piexif.ImageIFD.Model]: selection.setup.camera.model || 'Unknown',
        [piexif.ImageIFD.Artist]: selection.photographer || 'Unknown'
      },
      'Exif': {
        [piexif.ExifIFD.ISOSpeedRatings]: [selection.film.iso]
      },
      'GPS': {},
      'Interop': {},
      '1st': {},
      'thumbnail': null
    };

    // Add shot ISO if different and valid
    if (selection.shotISO && selection.shotISO !== selection.film.iso && typeof selection.shotISO === 'number') {
      minimalExifObj.Exif[piexif.ExifIFD.ISOSpeed] = selection.shotISO;
    }

    return piexif.dump(minimalExifObj);
  }

  async eraseExifFromFile(filePath) {
    try {
      const data = fs.readFileSync(filePath);
      const jpegData = data.toString('binary');

      const newJpegData = piexif.remove(jpegData);
      const newBuffer = Buffer.from(newJpegData, 'binary');
      fs.writeFileSync(filePath, newBuffer);

      return true;
    } catch (error) {
      console.error(`Error erasing EXIF from JPEG ${filePath}:`, error.message);
      return false;
    }
  }
}
