import { test, describe } from 'node:test';
import assert from 'node:assert';
import fs from 'fs';
import { generateXmpSidecar, createSidecarFile, removeSidecarFile } from '../src/exif/sidecarProcessor.js';

describe('Sidecar Processor', () => {
  const mockSelection = {
    setup: {
      camera: { maker: 'Canon', model: '5D' },
      lens: {
        maker: 'Canon',
        getLensModelWithAperture: () => '50mm f/1.4'
      }
    },
    film: { maker: 'Kodak', name: 'Portra 400', iso: 400 },
    photographer: 'Test Photographer',
    shotISO: 800
  };

  test('generateXmpSidecar should create valid XMP content', () => {
    const xmpContent = generateXmpSidecar(mockSelection);

    assert(xmpContent.includes('<?xml version="1.0" encoding="UTF-8"?>'));
    assert(xmpContent.includes('<tiff:Make>Canon</tiff:Make>'));
    assert(xmpContent.includes('<tiff:Model>5D</tiff:Model>'));
    assert(xmpContent.includes('<exif:LensMake>Canon</exif:LensMake>'));
    assert(xmpContent.includes('<exif:LensModel>50mm f/1.4</exif:LensModel>'));
    assert(xmpContent.includes('<rdf:li>400</rdf:li>'));
    assert(xmpContent.includes('<exif:PhotographicSensitivity>800</exif:PhotographicSensitivity>'));
    assert(xmpContent.includes('<rdf:li>Test Photographer</rdf:li>'));
  });

  test('generateXmpSidecar should handle missing shotISO', () => {
    const selectionNoShotISO = { ...mockSelection };
    delete selectionNoShotISO.shotISO;

    const xmpContent = generateXmpSidecar(selectionNoShotISO);

    assert(!xmpContent.includes('<exif:PhotographicSensitivity>'));
  });

  test('createSidecarFile should create XMP file', async() => {
    const testFile = '/tmp/test-image.jpg';
    const sidecarFile = testFile + '.xmp';

    // Clean up if exists
    if (fs.existsSync(sidecarFile)) {
      fs.unlinkSync(sidecarFile);
    }

    const result = await createSidecarFile(testFile, mockSelection);

    assert.strictEqual(result, true);
    assert(fs.existsSync(sidecarFile));

    const content = fs.readFileSync(sidecarFile, 'utf8');
    assert(content.includes('<tiff:Make>Canon</tiff:Make>'));

    // Clean up
    fs.unlinkSync(sidecarFile);
  });

  test('removeSidecarFile should remove existing XMP file', () => {
    const testFile = '/tmp/test-remove.jpg';
    const sidecarFile = testFile + '.xmp';

    // Create test sidecar file
    fs.writeFileSync(sidecarFile, 'test content');

    const result = removeSidecarFile(testFile);

    assert.strictEqual(result, true);
    assert(!fs.existsSync(sidecarFile));
  });

  test('removeSidecarFile should return false for non-existent file', () => {
    const testFile = '/tmp/non-existent.jpg';

    const result = removeSidecarFile(testFile);

    assert.strictEqual(result, false);
  });
});
