import { test, describe } from 'node:test';
import assert from 'node:assert';
import { JpegProcessor } from '../src/exif/jpegProcessor.js';

describe('JPEG Processor', () => {
  const processor = new JpegProcessor();

  const mockSelection = {
    setup: {
      camera: { maker: 'Leica', model: 'M7' },
      lens: {
        maker: 'Leica',
        model: 'Summicron',
        focalLength: '35',
        aperture: '2',
        getLensModelWithAperture: () => 'Summicron 35mm f/2'
      }
    },
    film: { maker: 'Harman', name: 'Phoenix II', iso: 200 },
    photographer: 'Test Photographer',
    shotISO: 400
  };

  test('validateSelection should pass with valid selection', () => {
    assert.doesNotThrow(() => {
      processor.validateSelection(mockSelection);
    });
  });

  test('validateSelection should throw with missing selection', () => {
    assert.throws(() => {
      processor.validateSelection(null);
    }, /Selection object is required/);
  });

  test('validateSelection should throw with missing setup', () => {
    const invalidSelection = { ...mockSelection };
    delete invalidSelection.setup;

    assert.throws(() => {
      processor.validateSelection(invalidSelection);
    }, /Selection must include setup with camera and lens/);
  });

  test('validateSelection should throw with missing camera', () => {
    const invalidSelection = {
      ...mockSelection,
      setup: { lens: mockSelection.setup.lens }
    };

    assert.throws(() => {
      processor.validateSelection(invalidSelection);
    }, /Selection must include setup with camera and lens/);
  });

  test('validateSelection should throw with missing film', () => {
    const invalidSelection = { ...mockSelection };
    delete invalidSelection.film;

    assert.throws(() => {
      processor.validateSelection(invalidSelection);
    }, /Selection must include film information/);
  });

  test('setAllExifValues should handle all EXIF fields', () => {
    const exifObj = {
      '0th': {},
      'Exif': {},
      'GPS': {},
      'Interop': {},
      '1st': {},
      'thumbnail': null
    };

    processor.setAllExifValues(exifObj, mockSelection);

    // Check that values were set
    assert(exifObj['0th'][271]); // Make
    assert(exifObj['0th'][272]); // Model
    assert(exifObj['0th'][315]); // Artist
    assert(exifObj['Exif'][34855]); // ISOSpeedRatings
    assert(exifObj['Exif'][34867]); // ISOSpeed (shot ISO)
    assert(exifObj['Exif'][42035]); // LensMake
    assert(exifObj['Exif'][42036]); // LensModel
  });

  test('setAllExifValues should handle missing shot ISO', () => {
    const exifObj = {
      '0th': {},
      'Exif': {},
      'GPS': {},
      'Interop': {},
      '1st': {},
      'thumbnail': null
    };

    const selectionNoShotISO = { ...mockSelection };
    delete selectionNoShotISO.shotISO;

    processor.setAllExifValues(exifObj, selectionNoShotISO);

    // Shot ISO should not be set
    assert(!exifObj['Exif'][34867]);
    // But film ISO should be set
    assert(exifObj['Exif'][34855]);
  });

  test('setAllExifValues should handle same shot and film ISO', () => {
    const exifObj = {
      '0th': {},
      'Exif': {},
      'GPS': {},
      'Interop': {},
      '1st': {},
      'thumbnail': null
    };

    const selectionSameISO = { ...mockSelection, shotISO: 200 }; // Same as film ISO

    processor.setAllExifValues(exifObj, selectionSameISO);

    // Shot ISO should not be set when it equals film ISO
    assert(!exifObj['Exif'][34867]);
    // But film ISO should be set
    assert(exifObj['Exif'][34855]);
  });
});
