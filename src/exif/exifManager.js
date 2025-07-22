import {
  getFileType,
  getFileExtension,
  isJpegFile,
  isTiffFile,
  isDngFile,
  isRawFile,
  isSupportedFile
} from './fileTypes.js';
import { JpegProcessor } from './jpegProcessor.js';
import { TiffProcessor } from './tiffProcessor.js';
import { RawProcessor } from './rawProcessor.js';
import { FileCollector } from './fileCollector.js';

/**
 * Main EXIF Manager - orchestrates different processors
 */
export class ExifManager {
  constructor() {
    this.jpegProcessor = new JpegProcessor();
    this.tiffProcessor = new TiffProcessor();
    this.rawProcessor = new RawProcessor();
    this.fileCollector = new FileCollector();
  }

  async applyExifToFile(filePath, selection) {
    const fileType = getFileType(filePath);

    switch (fileType) {
      case 'jpeg':
        return await this.jpegProcessor.applyExifToFile(filePath, selection);
      case 'tiff':
        return await this.tiffProcessor.applyExifToFile(filePath, selection);
      case 'dng':
      case 'raw':
        return await this.rawProcessor.applyExifToFile(filePath, selection);
      default:
        console.error(`Unsupported file type: ${fileType}`);
        return false;
    }
  }

  async eraseExifFromFile(filePath) {
    const fileType = getFileType(filePath);

    switch (fileType) {
      case 'jpeg':
        return await this.jpegProcessor.eraseExifFromFile(filePath);
      case 'tiff':
        return await this.tiffProcessor.eraseExifFromFile(filePath);
      case 'dng':
      case 'raw':
        return await this.rawProcessor.eraseExifFromFile(filePath);
      default:
        console.error(`Unsupported file type for EXIF removal: ${fileType}`);
        return false;
    }
  }

  collectFilesRecursively(folderPath, recursive = false) {
    return this.fileCollector.collectFilesRecursively(folderPath, recursive);
  }

  async processFolder(folderPath, selection, action, recursive = false) {
    return await this.fileCollector.processFolder(folderPath, selection, action, recursive);
  }

  // Legacy methods for backward compatibility
  getFileExtension(filePath) {
    return getFileExtension(filePath);
  }

  isJpegFile(filePath) {
    return isJpegFile(filePath);
  }

  isTiffFile(filePath) {
    return isTiffFile(filePath);
  }

  isDngFile(filePath) {
    return isDngFile(filePath);
  }

  isRawFile(filePath) {
    return isRawFile(filePath);
  }

  isSupportedFile(filePath) {
    return isSupportedFile(filePath);
  }

  getFileType(filePath) {
    return getFileType(filePath);
  }
}
