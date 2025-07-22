import path from 'path';

/**
 * File type detection utilities
 */

export function getFileExtension(filePath) {
  const fileName = path.basename(filePath);
  const lastDotIndex = fileName.lastIndexOf('.');
  if (lastDotIndex === -1 || lastDotIndex === fileName.length - 1) {
    return '';
  }
  return fileName.slice(lastDotIndex + 1).toLowerCase();
}

export function isJpegFile(filePath) {
  const ext = getFileExtension(filePath);
  return ['jpg', 'jpeg'].includes(ext);
}

export function isTiffFile(filePath) {
  const ext = getFileExtension(filePath);
  return ['tiff', 'tif'].includes(ext);
}

export function isDngFile(filePath) {
  const ext = getFileExtension(filePath);
  return ['dng'].includes(ext);
}

export function isRawFile(filePath) {
  const ext = getFileExtension(filePath);
  return ['dng', 'cr2', 'cr3', 'nef', 'nrw', 'arw', 'srf', 'sr2', 'orf', 'rw2', 'raf', 'srw', 'pef', 'x3f', 'erf', 'mef', 'mrw', 'dcr', 'kdc', '3fr', 'fff', 'iiq', 'k25', 'rwl'].includes(ext);
}

export function isSupportedFile(filePath) {
  return isJpegFile(filePath) || isTiffFile(filePath) || isDngFile(filePath) || isRawFile(filePath);
}

export function getFileType(filePath) {
  if (isJpegFile(filePath)) return 'jpeg';
  if (isTiffFile(filePath)) return 'tiff';
  if (isDngFile(filePath)) return 'dng';
  if (isRawFile(filePath)) return 'raw';
  return 'unknown';
}
