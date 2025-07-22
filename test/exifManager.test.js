import { test, describe, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert';
import fs from 'fs';
import path from 'path';
import os from 'os';
import { ExifManager } from '../src/exif.js';
import { Camera, Lens, Film, Setup } from '../src/models.js';

describe('ExifManager', () => {
  let exifManager;
  let testDir;

  beforeEach(() => {
    exifManager = new ExifManager();
    testDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ifex-exif-test-'));
  });

  afterEach(() => {
    // Clean up test files
    try {
      if (fs.existsSync(testDir)) {
        const files = fs.readdirSync(testDir);
        files.forEach(file => {
          fs.unlinkSync(path.join(testDir, file));
        });
        fs.rmdirSync(testDir);
      }
    } catch {
      // Ignore cleanup errors
    }
  });

  describe('EXIF Tags Configuration', () => {
    test('should have correct EXIF tag mappings', () => {
      assert.ok(exifManager.exifTags);
      assert.ok(exifManager.exifTags.cameraMaker);
      assert.ok(exifManager.exifTags.cameraModel);
      assert.ok(exifManager.exifTags.lensMaker);
      assert.ok(exifManager.exifTags.lensModel);
      assert.ok(exifManager.exifTags.filmISO);
      assert.ok(exifManager.exifTags.shotISO);
      assert.ok(exifManager.exifTags.photographer);
    });
  });

  describe('File Type Detection', () => {
    test('should detect JPEG files correctly', () => {
      assert.strictEqual(exifManager.isJpegFile('photo.jpg'), true);
      assert.strictEqual(exifManager.isJpegFile('photo.jpeg'), true);
      assert.strictEqual(exifManager.isJpegFile('PHOTO.JPG'), true);
      assert.strictEqual(exifManager.isJpegFile('PHOTO.JPEG'), true);
      assert.strictEqual(exifManager.isJpegFile('photo.png'), false);
      assert.strictEqual(exifManager.isJpegFile('photo.gif'), false);
      assert.strictEqual(exifManager.isJpegFile('photo.tiff'), false);
      assert.strictEqual(exifManager.isJpegFile('document.pdf'), false);
    });

    test('should detect TIFF files correctly', () => {
      assert.strictEqual(exifManager.isTiffFile('image.tiff'), true);
      assert.strictEqual(exifManager.isTiffFile('image.tif'), true);
      assert.strictEqual(exifManager.isTiffFile('IMAGE.TIFF'), true);
      assert.strictEqual(exifManager.isTiffFile('IMAGE.TIF'), true);
      assert.strictEqual(exifManager.isTiffFile('image.jpg'), false);
      assert.strictEqual(exifManager.isTiffFile('image.dng'), false);
    });

    test('should detect DNG files correctly', () => {
      assert.strictEqual(exifManager.isDngFile('raw.dng'), true);
      assert.strictEqual(exifManager.isDngFile('RAW.DNG'), true);
      assert.strictEqual(exifManager.isDngFile('raw.cr2'), false);
      assert.strictEqual(exifManager.isDngFile('raw.jpg'), false);
    });

    test('should detect RAW files correctly', () => {
      assert.strictEqual(exifManager.isRawFile('photo.dng'), true);
      assert.strictEqual(exifManager.isRawFile('photo.cr2'), true);
      assert.strictEqual(exifManager.isRawFile('photo.nef'), true);
      assert.strictEqual(exifManager.isRawFile('photo.arw'), true);
      assert.strictEqual(exifManager.isRawFile('PHOTO.CR2'), true);
      assert.strictEqual(exifManager.isRawFile('photo.jpg'), false);
      assert.strictEqual(exifManager.isRawFile('photo.tiff'), false);
    });

    test('should detect supported files correctly', () => {
      assert.strictEqual(exifManager.isSupportedFile('photo.jpg'), true);
      assert.strictEqual(exifManager.isSupportedFile('photo.tiff'), true);
      assert.strictEqual(exifManager.isSupportedFile('photo.dng'), true);
      assert.strictEqual(exifManager.isSupportedFile('photo.cr2'), true);
      assert.strictEqual(exifManager.isSupportedFile('photo.png'), false);
      assert.strictEqual(exifManager.isSupportedFile('document.pdf'), false);
    });

    test('should return correct file type', () => {
      assert.strictEqual(exifManager.getFileType('photo.jpg'), 'jpeg');
      assert.strictEqual(exifManager.getFileType('image.tiff'), 'tiff');
      assert.strictEqual(exifManager.getFileType('raw.dng'), 'dng');
      assert.strictEqual(exifManager.getFileType('raw.cr2'), 'raw');
      assert.strictEqual(exifManager.getFileType('image.png'), 'unknown');
    });
  });

  describe('setExifValue Method', () => {
    test('should set EXIF value when value is provided', () => {
      const exifObj = {
        '0th': {},
        'Exif': {}
      };

      exifManager.setExifValue(exifObj, 'cameraMaker', 'Canon');

      // Check that the value was set (we can't easily test the exact piexif constants)
      assert.ok(Object.keys(exifObj['0th']).length > 0);
    });

    test('should not set EXIF value when value is empty', () => {
      const exifObj = {
        '0th': {},
        'Exif': {}
      };

      exifManager.setExifValue(exifObj, 'cameraMaker', '');
      exifManager.setExifValue(exifObj, 'cameraMaker', null);
      exifManager.setExifValue(exifObj, 'cameraMaker', undefined);

      assert.strictEqual(Object.keys(exifObj['0th']).length, 0);
    });
  });

  describe('processFolder Method', () => {
    test('should return error for non-existent folder', async() => {
      const selection = createTestSelection();
      const result = await exifManager.processFolder('/non/existent/path', selection, 'apply');

      assert.strictEqual(result.success, false);
      assert.ok(result.message);
    });

    test('should return message when no supported image files found', async() => {
      // Create a folder with non-JPEG files
      fs.writeFileSync(path.join(testDir, 'test.txt'), 'test content');
      fs.writeFileSync(path.join(testDir, 'image.png'), 'fake png content');

      const selection = createTestSelection();
      const result = await exifManager.processFolder(testDir, selection, 'apply');

      assert.strictEqual(result.success, false);
      assert.ok(result.message.includes('No supported image files'));
    });

    test('should identify supported image files in folder', async() => {
      // Create fake image files (just for file detection testing)
      fs.writeFileSync(path.join(testDir, 'photo1.jpg'), 'fake jpeg content');
      fs.writeFileSync(path.join(testDir, 'photo2.jpeg'), 'fake jpeg content');
      fs.writeFileSync(path.join(testDir, 'photo3.png'), 'fake png content');

      const selection = createTestSelection();
      const result = await exifManager.processFolder(testDir, selection, 'apply');

      // Even though EXIF processing will fail (fake files), folder processing should detect JPEG files
      assert.strictEqual(result.success, true);
      assert.strictEqual(result.results.total, 2); // Only .jpg and .jpeg files
    });
  });

  describe('Selection Data Structure', () => {
    test('should work with complete selection object', () => {
      const selection = createTestSelection();

      assert.ok(selection.setup);
      assert.ok(selection.setup.camera);
      assert.ok(selection.setup.lens);
      assert.ok(selection.film);
      assert.ok(selection.shotISO);
      assert.ok(selection.photographer);

      // Test lens methods are callable
      assert.ok(selection.setup.lens.getLensModelWithAperture());
      assert.ok(selection.setup.lens.getDisplayName());
    });
  });

  // Helper function to create test selection data
  function createTestSelection() {
    const camera = new Camera('Canon', 'EOS R5');
    const lens = new Lens('Canon', 'L', '50', '1.2', 'EF');
    const film = new Film('Kodak', 'Portra', 400);
    const setup = new Setup('Test Setup', camera.id, lens.id);

    return {
      setup: {
        setup,
        camera,
        lens
      },
      film,
      shotISO: 800,
      photographer: 'Test Photographer'
    };
  }
});
