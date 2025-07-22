import { TiffProcessor } from './tiffProcessor.js';
import { createSidecarFile, removeSidecarFile } from './sidecarProcessor.js';
import { isDngFile } from './fileTypes.js';

/**
 * RAW file processing
 */
export class RawProcessor {
  constructor() {
    this.tiffProcessor = new TiffProcessor();
  }

  async applyExifToFile(filePath, selection) {
    try {
      this.validateSelection(selection);

      // For DNG files (which are TIFF-based), try TIFF approach first
      if (isDngFile(filePath)) {
        return await this.tiffProcessor.applyExifToFile(filePath, selection);
      }

      // For other RAW formats, we'll create a sidecar file approach
      // Since most RAW formats are proprietary and read-only
      console.warn(`RAW file format detected for ${filePath}. Creating sidecar metadata file instead.`);
      return await createSidecarFile(filePath, selection);

    } catch (error) {
      console.error(`Error processing RAW file ${filePath}:`, error.message);
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

  async eraseExifFromFile(filePath) {
    try {
      // For DNG files, try to erase EXIF using TIFF method
      if (isDngFile(filePath)) {
        return await this.tiffProcessor.eraseExifFromFile(filePath);
      }

      // For other RAW formats, remove sidecar files if they exist
      const removed = removeSidecarFile(filePath);

      if (removed) {
        return true;
      } else {
        console.warn(`RAW file format detected for ${filePath}. Only sidecar files can be removed.`);
        console.warn('Direct EXIF modification of proprietary RAW formats is not supported.');
        return true;
      }
    } catch (error) {
      console.error(`Error erasing EXIF from RAW file ${filePath}:`, error.message);
      return false;
    }
  }
}
