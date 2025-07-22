import fs from 'fs';
import UTIF from 'utif';
import { createSidecarFile } from './sidecarProcessor.js';

/**
 * TIFF EXIF processing
 */
export class TiffProcessor {
  async applyExifToFile(filePath, selection) {
    try {
      this.validateSelection(selection);

      const data = fs.readFileSync(filePath);
      let tiffs;

      try {
        tiffs = UTIF.decode(data);
      } catch (utifError) {
        console.warn(`UTIF decode failed for ${filePath}, creating sidecar file instead:`, utifError.message);
        return await createSidecarFile(filePath, selection);
      }

      if (tiffs.length === 0) {
        throw new Error('No valid TIFF data found');
      }

      const tiff = tiffs[0];

      try {
        this.setTiffExifValues(tiff, selection);
        const newData = UTIF.encode([tiff]);
        fs.writeFileSync(filePath, Buffer.from(newData));
        return true;
      } catch (encodeError) {
        console.warn(`UTIF encode failed for ${filePath}, creating sidecar file instead:`, encodeError.message);
        return await createSidecarFile(filePath, selection);
      }
    } catch (error) {
      console.error(`Error processing TIFF ${filePath}:`, error.message);
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

  setTiffExifValues(tiff, selection) {
    // Apply EXIF data to TIFF
    tiff.t256 = tiff.t256 || tiff.width; // ImageWidth
    tiff.t257 = tiff.t257 || tiff.height; // ImageLength

    // Camera information
    tiff.t271 = selection.setup.camera.maker; // Make
    tiff.t272 = selection.setup.camera.model; // Model

    // Lens information
    tiff.t42035 = selection.setup.lens.maker; // LensMake
    tiff.t42036 = selection.setup.lens.getLensModelWithAperture(); // LensModel

    // Film/ISO information
    tiff.t34855 = [selection.film.iso]; // ISOSpeedRatings
    if (selection.shotISO) {
      tiff.t34864 = selection.shotISO; // SensitivityType
    }

    // Photographer
    tiff.t315 = selection.photographer; // Artist
  }

  async eraseExifFromFile(filePath) {
    try {
      const data = fs.readFileSync(filePath);
      const tiffs = UTIF.decode(data);

      if (tiffs.length === 0) {
        throw new Error('No valid TIFF data found');
      }

      const tiff = tiffs[0];

      // Remove EXIF-related tags while preserving essential image data
      const essentialTags = ['t256', 't257', 't258', 't259', 't262', 't273', 't277', 't278', 't279'];
      const cleanTiff = {};

      // Keep only essential tags for image structure
      essentialTags.forEach(tag => {
        if (tiff[tag] !== undefined) {
          cleanTiff[tag] = tiff[tag];
        }
      });

      // Preserve width/height if they exist as properties
      if (tiff.width) cleanTiff.width = tiff.width;
      if (tiff.height) cleanTiff.height = tiff.height;
      if (tiff.data) cleanTiff.data = tiff.data;

      const newData = UTIF.encode([cleanTiff]);
      fs.writeFileSync(filePath, Buffer.from(newData));

      return true;
    } catch (error) {
      console.error(`Error erasing EXIF from TIFF ${filePath}:`, error.message);
      return false;
    }
  }
}
