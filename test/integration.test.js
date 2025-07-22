import { test, describe, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert';
import fs from 'fs';
import path from 'path';
import os from 'os';
import { DataManager } from '../src/dataManager.js';
import { ExifManager } from '../src/exif.js';

describe('Integration Tests', () => {
  let dataManager;
  let exifManager;
  let testDir;

  beforeEach(() => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ifex-integration-test-'));
    const testDataFile = path.join(tempDir, 'ifex-test.json');

    dataManager = new DataManager(testDataFile);
    exifManager = new ExifManager();
    testDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ifex-integration-test-'));
  });

  afterEach(() => {
    // Clean up
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

  describe('Complete Workflow', () => {
    test('should create equipment and setup, then process folder', async() => {
      // Step 1: Create equipment
      const camera = dataManager.createCamera('Canon', 'AE-1');
      const lens = dataManager.createLens('Canon', 'FD', '50', '1.2', 'FD');
      const film = dataManager.createFilm('Kodak', 'Portra', 400);

      assert.ok(camera);
      assert.ok(lens);
      assert.ok(film);

      // Step 2: Create setup
      const setup = dataManager.createSetup('Portrait Setup', camera.id, lens.id);
      assert.ok(setup);

      // Step 3: Get setup with details (simulating CLI workflow)
      const setupDetails = dataManager.getSetupWithDetails(setup.id);
      assert.ok(setupDetails);
      assert.ok(setupDetails.camera);
      assert.ok(setupDetails.lens);

      // Step 4: Create selection object (as CLI would)
      const selection = {
        setup: setupDetails,
        film: film,
        shotISO: 800,
        photographer: 'Test Photographer'
      };

      // Verify selection structure
      assert.strictEqual(selection.setup.camera.maker, 'Canon');
      assert.strictEqual(selection.setup.lens.focalLength, '50');
      assert.strictEqual(selection.film.name, 'Portra');

      // Step 5: Test lens display methods
      assert.strictEqual(selection.setup.lens.getDisplayName(), 'Canon 50mm f/1.2 (FD)');
      assert.strictEqual(selection.setup.lens.getLensModelWithAperture(), '50mm f/1.2');

      // Step 6: Test folder processing (without actual image files)
      const result = await exifManager.processFolder(testDir, selection, 'apply');
      assert.strictEqual(result.success, false); // No image files
      assert.ok(result.message.includes('No supported image files'));
    });

    test('should prevent deletion of equipment used in setups', () => {
      // Create equipment and setup
      const camera = dataManager.createCamera('Canon', 'AE-1');
      const lens = dataManager.createLens('Canon', 'FD', '85', '1.4', 'FD');
      const setup = dataManager.createSetup('Portrait Setup', camera.id, lens.id);

      // Try to delete camera used in setup
      const cameraDeleteResult = dataManager.deleteCamera(camera.id);
      assert.strictEqual(cameraDeleteResult.success, false);
      assert.ok(cameraDeleteResult.message.includes('used in existing setups'));

      // Try to delete lens used in setup
      const lensDeleteResult = dataManager.deleteLens(lens.id);
      assert.strictEqual(lensDeleteResult.success, false);
      assert.ok(lensDeleteResult.message.includes('used in existing setups'));

      // Delete setup first, then equipment should be deletable
      dataManager.deleteSetup(setup.id);

      const cameraDeleteResult2 = dataManager.deleteCamera(camera.id);
      assert.strictEqual(cameraDeleteResult2.success, true);

      const lensDeleteResult2 = dataManager.deleteLens(lens.id);
      assert.strictEqual(lensDeleteResult2.success, true);
    });

    test('should handle lens without focal length or model gracefully', () => {
      const camera = dataManager.createCamera('Canon', 'AE-1');

      // Create lens with only model (no focal length)
      const lens1 = dataManager.createLens('Canon', 'Macro', null, '2.8', 'FD');
      assert.strictEqual(lens1.getDisplayName(), 'Canon Macro f/2.8 (FD)');
      assert.strictEqual(lens1.getLensModelWithAperture(), 'Macro f/2.8');

      // Create lens with only focal length (no model)
      const lens2 = dataManager.createLens('Nikon', null, '35', '1.4', 'F');
      assert.strictEqual(lens2.getDisplayName(), 'Nikon 35mm f/1.4 (F)');
      assert.strictEqual(lens2.getLensModelWithAperture(), '35mm f/1.4');

      // Both should be able to create setups
      const setup1 = dataManager.createSetup('Macro Setup', camera.id, lens1.id);
      const setup2 = dataManager.createSetup('Wide Setup', camera.id, lens2.id);

      assert.ok(setup1);
      assert.ok(setup2);
    });

    test('should validate zoom lens ranges', () => {
      const camera = dataManager.createCamera('Canon', 'AE-1');
      const zoomLens = dataManager.createLens('Canon', 'FD', '24-70', '2.8', 'FD');

      assert.strictEqual(zoomLens.focalLength, '24-70');
      assert.strictEqual(zoomLens.getDisplayName(), 'Canon 24-70mm f/2.8 (FD)');
      assert.strictEqual(zoomLens.getLensModelWithAperture(), '24-70mm f/2.8');

      const setup = dataManager.createSetup('Versatile Setup', camera.id, zoomLens.id);
      assert.ok(setup);
    });

    test('should handle complete CRUD lifecycle', () => {
      // Create
      const camera = dataManager.createCamera('Nikon', 'FM2');
      const lens = dataManager.createLens('Nikon', 'AI', '85', '1.4', 'F');
      const film = dataManager.createFilm('Fuji', 'Velvia', 50);
      const setup = dataManager.createSetup('Portrait', camera.id, lens.id);

      // Read
      assert.strictEqual(dataManager.getCameras().length, 1);
      assert.strictEqual(dataManager.getLenses().length, 1);
      assert.strictEqual(dataManager.getFilms().length, 1);
      assert.strictEqual(dataManager.getSetups().length, 1);

      // Update
      dataManager.updateCamera(camera.id, 'Nikon', 'FE2');
      dataManager.updateLens(lens.id, 'Nikon', 'AI-S', '85', '1.2', 'F');
      dataManager.updateFilm(film.id, 'Fuji', 'Provia', 100);
      dataManager.updateSetup(setup.id, 'Studio Portrait', camera.id, lens.id);

      // Verify updates
      const updatedCamera = dataManager.getCameraById(camera.id);
      const updatedLens = dataManager.getLensById(lens.id);
      const updatedFilm = dataManager.getFilmById(film.id);
      const updatedSetup = dataManager.getSetupById(setup.id);

      assert.strictEqual(updatedCamera.model, 'FE2');
      assert.strictEqual(updatedLens.aperture, '1.2');
      assert.strictEqual(updatedLens.mount, 'F');
      assert.strictEqual(updatedFilm.iso, 100);
      assert.strictEqual(updatedSetup.name, 'Studio Portrait');

      // Delete (setup first, then equipment)
      dataManager.deleteSetup(setup.id);
      dataManager.deleteCamera(camera.id);
      dataManager.deleteLens(lens.id);
      dataManager.deleteFilm(film.id);

      // Verify deletions
      assert.strictEqual(dataManager.getCameras().length, 0);
      assert.strictEqual(dataManager.getLenses().length, 0);
      assert.strictEqual(dataManager.getFilms().length, 0);
      assert.strictEqual(dataManager.getSetups().length, 0);
    });
  });
});
