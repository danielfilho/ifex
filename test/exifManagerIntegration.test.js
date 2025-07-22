import { test, describe } from 'node:test';
import assert from 'node:assert';
import { ExifManager } from '../src/exif/exifManager.js';

describe('ExifManager Integration', () => {
  const manager = new ExifManager();

  test('should create ExifManager instance', () => {
    assert(manager instanceof ExifManager);
  });

  test('should have all required methods', () => {
    assert(typeof manager.applyExifToFile === 'function');
    assert(typeof manager.eraseExifFromFile === 'function');
    assert(typeof manager.processFolder === 'function');
    assert(typeof manager.collectFilesRecursively === 'function');
  });

  test('should have legacy compatibility methods', () => {
    assert(typeof manager.getFileExtension === 'function');
    assert(typeof manager.isJpegFile === 'function');
    assert(typeof manager.isTiffFile === 'function');
    assert(typeof manager.isDngFile === 'function');
    assert(typeof manager.isRawFile === 'function');
    assert(typeof manager.isSupportedFile === 'function');
    assert(typeof manager.getFileType === 'function');
  });

  test('legacy methods should work correctly', () => {
    assert.strictEqual(manager.getFileExtension('/path/to/file.jpg'), 'jpg');
    assert.strictEqual(manager.isJpegFile('/path/to/image.jpg'), true);
    assert.strictEqual(manager.isTiffFile('/path/to/image.tiff'), true);
    assert.strictEqual(manager.isDngFile('/path/to/image.dng'), true);
    assert.strictEqual(manager.isRawFile('/path/to/image.cr2'), true);
    assert.strictEqual(manager.isSupportedFile('/path/to/image.jpg'), true);
    assert.strictEqual(manager.getFileType('/path/to/image.jpg'), 'jpeg');
  });

  test('getFileType should delegate correctly', () => {
    assert.strictEqual(manager.getFileType('/path/test.jpg'), 'jpeg');
    assert.strictEqual(manager.getFileType('/path/test.tiff'), 'tiff');
    assert.strictEqual(manager.getFileType('/path/test.dng'), 'dng');
    assert.strictEqual(manager.getFileType('/path/test.cr2'), 'raw');
    assert.strictEqual(manager.getFileType('/path/test.png'), 'unknown');
  });

  test('should have processor instances', () => {
    assert(manager.jpegProcessor);
    assert(manager.tiffProcessor);
    assert(manager.rawProcessor);
    assert(manager.fileCollector);
  });
});
