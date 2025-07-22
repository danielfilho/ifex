import { test, describe } from 'node:test';
import assert from 'node:assert';
import { EXIF_TAGS, setExifValue, createCleanExifObject } from '../src/exif/tags.js';

describe('EXIF Tags', () => {
  test('EXIF_TAGS should contain all required tag definitions', () => {
    assert(EXIF_TAGS.cameraMaker);
    assert(EXIF_TAGS.cameraModel);
    assert(EXIF_TAGS.lensMaker);
    assert(EXIF_TAGS.lensModel);
    assert(EXIF_TAGS.focalLength);
    assert(EXIF_TAGS.fNumber);
    assert(EXIF_TAGS.filmISO);
    assert(EXIF_TAGS.shotISO);
    assert(EXIF_TAGS.photographer);
  });

  test('setExifValue should set values correctly', () => {
    const exifObj = createCleanExifObject();

    setExifValue(exifObj, 'cameraMaker', 'Leica');
    setExifValue(exifObj, 'cameraModel', 'M7');
    setExifValue(exifObj, 'photographer', 'John Doe');

    assert.strictEqual(exifObj['0th'][271], 'Leica'); // Make tag
    assert.strictEqual(exifObj['0th'][272], 'M7'); // Model tag
    assert.strictEqual(exifObj['0th'][315], 'John Doe'); // Artist tag
  });

  test('setExifValue should handle empty values', () => {
    const exifObj = createCleanExifObject();

    setExifValue(exifObj, 'cameraMaker', '');
    setExifValue(exifObj, 'cameraModel', null);
    setExifValue(exifObj, 'photographer', undefined);

    assert.strictEqual(Object.keys(exifObj['0th']).length, 0);
  });

  test('setExifValue should create IFD sections if missing', () => {
    const exifObj = {};

    setExifValue(exifObj, 'cameraMaker', 'Canon');
    setExifValue(exifObj, 'filmISO', [200]);

    assert(exifObj['0th']);
    assert(exifObj['Exif']);
    assert.strictEqual(exifObj['0th'][271], 'Canon');
    assert.deepStrictEqual(exifObj['Exif'][34855], [200]);
  });

  test('createCleanExifObject should return proper structure', () => {
    const exifObj = createCleanExifObject();

    assert(typeof exifObj === 'object');
    assert(exifObj['0th']);
    assert(exifObj['Exif']);
    assert(exifObj['GPS']);
    assert(exifObj['Interop']);
    assert(exifObj['1st']);
    assert(exifObj.thumbnail === null);
  });
});
