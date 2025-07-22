import { test, describe } from 'node:test';
import assert from 'node:assert';
import {
  getFileExtension,
  isJpegFile,
  isTiffFile,
  isDngFile,
  isRawFile,
  isSupportedFile,
  getFileType
} from '../src/exif/fileTypes.js';

describe('File Types', () => {
  test('getFileExtension should extract extension correctly', () => {
    assert.strictEqual(getFileExtension('/path/to/file.jpg'), 'jpg');
    assert.strictEqual(getFileExtension('/path/to/file.JPEG'), 'jpeg');
    assert.strictEqual(getFileExtension('/path/to/file.tiff'), 'tiff');
    assert.strictEqual(getFileExtension('/path/to/file'), '');
    assert.strictEqual(getFileExtension('/path/to/.hidden'), '');
  });

  test('isJpegFile should detect JPEG files', () => {
    assert.strictEqual(isJpegFile('/path/to/image.jpg'), true);
    assert.strictEqual(isJpegFile('/path/to/image.jpeg'), true);
    assert.strictEqual(isJpegFile('/path/to/image.JPG'), true);
    assert.strictEqual(isJpegFile('/path/to/image.png'), false);
    assert.strictEqual(isJpegFile('/path/to/image.tiff'), false);
  });

  test('isTiffFile should detect TIFF files', () => {
    assert.strictEqual(isTiffFile('/path/to/image.tiff'), true);
    assert.strictEqual(isTiffFile('/path/to/image.tif'), true);
    assert.strictEqual(isTiffFile('/path/to/image.TIFF'), true);
    assert.strictEqual(isTiffFile('/path/to/image.jpg'), false);
  });

  test('isDngFile should detect DNG files', () => {
    assert.strictEqual(isDngFile('/path/to/image.dng'), true);
    assert.strictEqual(isDngFile('/path/to/image.DNG'), true);
    assert.strictEqual(isDngFile('/path/to/image.jpg'), false);
  });

  test('isRawFile should detect RAW files', () => {
    assert.strictEqual(isRawFile('/path/to/image.dng'), true);
    assert.strictEqual(isRawFile('/path/to/image.cr2'), true);
    assert.strictEqual(isRawFile('/path/to/image.nef'), true);
    assert.strictEqual(isRawFile('/path/to/image.arw'), true);
    assert.strictEqual(isRawFile('/path/to/image.jpg'), false);
  });

  test('isSupportedFile should detect all supported formats', () => {
    assert.strictEqual(isSupportedFile('/path/to/image.jpg'), true);
    assert.strictEqual(isSupportedFile('/path/to/image.tiff'), true);
    assert.strictEqual(isSupportedFile('/path/to/image.dng'), true);
    assert.strictEqual(isSupportedFile('/path/to/image.cr2'), true);
    assert.strictEqual(isSupportedFile('/path/to/image.png'), false);
    assert.strictEqual(isSupportedFile('/path/to/image.gif'), false);
  });

  test('getFileType should return correct type', () => {
    assert.strictEqual(getFileType('/path/to/image.jpg'), 'jpeg');
    assert.strictEqual(getFileType('/path/to/image.tiff'), 'tiff');
    assert.strictEqual(getFileType('/path/to/image.dng'), 'dng');
    assert.strictEqual(getFileType('/path/to/image.cr2'), 'raw');
    assert.strictEqual(getFileType('/path/to/image.png'), 'unknown');
  });
});
