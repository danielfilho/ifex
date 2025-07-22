import { test, describe, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert';
import fs from 'fs';
import path from 'path';
import os from 'os';
import { DataManager } from '../src/dataManager.js';

describe('DataManager', () => {
  let dataManager;
  let testDataFile;

  beforeEach(() => {
    // Create a temporary data file for testing
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ifex-test-'));
    testDataFile = path.join(tempDir, 'ifex-test.json');

    // Create DataManager instance with custom data file
    dataManager = new DataManager(testDataFile);
  });

  afterEach(() => {
    // Clean up test files
    try {
      if (fs.existsSync(testDataFile)) {
        fs.unlinkSync(testDataFile);
      }
      const testDir = path.dirname(testDataFile);
      if (fs.existsSync(testDir)) {
        fs.rmdirSync(testDir);
      }
    } catch {
      // Ignore cleanup errors
    }
  });

  describe('Camera CRUD', () => {
    test('should create a camera', () => {
      const camera = dataManager.createCamera('Canon', 'AE-1');

      assert.ok(camera);
      assert.strictEqual(camera.maker, 'Canon');
      assert.strictEqual(camera.model, 'AE-1');
      assert.ok(camera.id);
    });

    test('should get all cameras', () => {
      dataManager.createCamera('Canon', 'AE-1');
      dataManager.createCamera('Nikon', 'FM2');

      const cameras = dataManager.getCameras();

      assert.strictEqual(cameras.length, 2);
      assert.strictEqual(cameras[0].maker, 'Canon');
      assert.strictEqual(cameras[1].maker, 'Nikon');
    });

    test('should get camera by ID', () => {
      const created = dataManager.createCamera('Pentax', 'K1000');
      const found = dataManager.getCameraById(created.id);

      assert.ok(found);
      assert.strictEqual(found.id, created.id);
      assert.strictEqual(found.maker, 'Pentax');
    });

    test('should update camera', () => {
      const camera = dataManager.createCamera('Canon', 'AE-1');
      const success = dataManager.updateCamera(camera.id, 'Canon', 'A-1');

      assert.strictEqual(success, true);

      const updated = dataManager.getCameraById(camera.id);
      assert.strictEqual(updated.model, 'A-1');
    });

    test('should delete camera when not used in setups', () => {
      const camera = dataManager.createCamera('Canon', 'AE-1');
      const result = dataManager.deleteCamera(camera.id);

      assert.strictEqual(result.success, true);

      const found = dataManager.getCameraById(camera.id);
      assert.strictEqual(found, null);
    });
  });

  describe('Lens CRUD', () => {
    test('should create a lens with all fields', () => {
      const lens = dataManager.createLens('Canon', 'FD', '50', '1.2', 'FD');

      assert.ok(lens);
      assert.strictEqual(lens.maker, 'Canon');
      assert.strictEqual(lens.model, 'FD');
      assert.strictEqual(lens.focalLength, '50');
      assert.strictEqual(lens.aperture, '1.2');
      assert.strictEqual(lens.mount, 'FD');
    });

    test('should create lens with null optional fields', () => {
      const lens = dataManager.createLens('Nikon', null, '35', '1.4', 'F');

      assert.ok(lens);
      assert.strictEqual(lens.maker, 'Nikon');
      assert.strictEqual(lens.model, null);
      assert.strictEqual(lens.focalLength, '35');
    });

    test('should get all lenses', () => {
      dataManager.createLens('Canon', 'FD', '50', '1.2', 'FD');
      dataManager.createLens('Nikon', 'AI', '35', '1.4', 'F');

      const lenses = dataManager.getLenses();

      assert.strictEqual(lenses.length, 2);
    });

    test('should update lens', () => {
      const lens = dataManager.createLens('Canon', 'FD', '50', '1.2', 'FD');
      const success = dataManager.updateLens(lens.id, 'Canon', 'FD', '85', '1.2', 'FD');

      assert.strictEqual(success, true);

      const updated = dataManager.getLensById(lens.id);
      assert.strictEqual(updated.focalLength, '85');
    });
  });

  describe('Film CRUD', () => {
    test('should create a film', () => {
      const film = dataManager.createFilm('Kodak', 'Portra', 400);

      assert.ok(film);
      assert.strictEqual(film.maker, 'Kodak');
      assert.strictEqual(film.name, 'Portra');
      assert.strictEqual(film.iso, 400);
    });

    test('should get all films', () => {
      dataManager.createFilm('Kodak', 'Portra', 400);
      dataManager.createFilm('Fuji', 'Velvia', 50);

      const films = dataManager.getFilms();

      assert.strictEqual(films.length, 2);
    });

    test('should update film', () => {
      const film = dataManager.createFilm('Kodak', 'Gold', 200);
      const success = dataManager.updateFilm(film.id, 'Kodak', 'Gold', 400);

      assert.strictEqual(success, true);

      const updated = dataManager.getFilmById(film.id);
      assert.strictEqual(updated.iso, 400);
    });

    test('should delete film', () => {
      const film = dataManager.createFilm('Kodak', 'Gold', 200);
      const result = dataManager.deleteFilm(film.id);

      assert.strictEqual(result.success, true);

      const found = dataManager.getFilmById(film.id);
      assert.strictEqual(found, null);
    });
  });

  describe('Photographer CRUD', () => {
    test('should create a photographer with name only', () => {
      const photographer = dataManager.createPhotographer('John Doe');

      assert.ok(photographer);
      assert.strictEqual(photographer.name, 'John Doe');
      assert.strictEqual(photographer.email, null);
      assert.ok(photographer.id);
    });

    test('should create a photographer with name and email', () => {
      const photographer = dataManager.createPhotographer('Jane Smith', 'jane@example.com');

      assert.ok(photographer);
      assert.strictEqual(photographer.name, 'Jane Smith');
      assert.strictEqual(photographer.email, 'jane@example.com');
      assert.ok(photographer.id);
    });

    test('should get all photographers', () => {
      dataManager.createPhotographer('Alice Brown', 'alice@studio.com');
      dataManager.createPhotographer('Bob Wilson');

      const photographers = dataManager.getPhotographers();

      assert.strictEqual(photographers.length, 2);
      assert.strictEqual(photographers[0].name, 'Alice Brown');
      assert.strictEqual(photographers[1].name, 'Bob Wilson');
    });

    test('should get photographer by ID', () => {
      const created = dataManager.createPhotographer('Charlie Davis', 'charlie@photo.net');
      const found = dataManager.getPhotographerById(created.id);

      assert.ok(found);
      assert.strictEqual(found.id, created.id);
      assert.strictEqual(found.name, 'Charlie Davis');
      assert.strictEqual(found.email, 'charlie@photo.net');
    });

    test('should update photographer', () => {
      const photographer = dataManager.createPhotographer('Dave Miller');
      const success = dataManager.updatePhotographer(photographer.id, 'David Miller', 'david@photography.com');

      assert.strictEqual(success, true);

      const updated = dataManager.getPhotographerById(photographer.id);
      assert.strictEqual(updated.name, 'David Miller');
      assert.strictEqual(updated.email, 'david@photography.com');
    });

    test('should delete photographer', () => {
      const photographer = dataManager.createPhotographer('Eva Thompson', 'eva@photos.com');
      const result = dataManager.deletePhotographer(photographer.id);

      assert.strictEqual(result.success, true);

      const found = dataManager.getPhotographerById(photographer.id);
      assert.strictEqual(found, null);
    });
  });

  describe('Setup CRUD', () => {
    test('should create setup with valid camera and lens', () => {
      const camera = dataManager.createCamera('Canon', 'AE-1');
      const lens = dataManager.createLens('Canon', 'FD', '50', '1.2', 'FD');

      const setup = dataManager.createSetup('Portrait Setup', camera.id, lens.id);

      assert.ok(setup);
      assert.strictEqual(setup.name, 'Portrait Setup');
      assert.strictEqual(setup.cameraId, camera.id);
      assert.strictEqual(setup.lensId, lens.id);
    });

    test('should not create setup with invalid camera or lens', () => {
      const setup = dataManager.createSetup('Invalid Setup', 'invalid-camera', 'invalid-lens');

      assert.strictEqual(setup, null);
    });

    test('should get setup with details', () => {
      const camera = dataManager.createCamera('Canon', 'AE-1');
      const lens = dataManager.createLens('Canon', 'FD', '50', '1.2', 'FD');
      const setup = dataManager.createSetup('Test Setup', camera.id, lens.id);

      const details = dataManager.getSetupWithDetails(setup.id);

      assert.ok(details);
      assert.ok(details.setup);
      assert.ok(details.camera);
      assert.ok(details.lens);
      assert.strictEqual(details.setup.name, 'Test Setup');
      assert.strictEqual(details.camera.maker, 'Canon');
      assert.strictEqual(details.lens.maker, 'Canon');
    });

    test('should prevent deleting camera used in setup', () => {
      const camera = dataManager.createCamera('Canon', 'AE-1');
      const lens = dataManager.createLens('Canon', 'FD', '50', '1.2', 'FD');
      dataManager.createSetup('Test Setup', camera.id, lens.id);

      const result = dataManager.deleteCamera(camera.id);

      assert.strictEqual(result.success, false);
      assert.ok(result.message.includes('used in existing setups'));
    });

    test('should prevent deleting lens used in setup', () => {
      const camera = dataManager.createCamera('Canon', 'AE-1');
      const lens = dataManager.createLens('Canon', 'FD', '50', '1.2', 'FD');
      dataManager.createSetup('Test Setup', camera.id, lens.id);

      const result = dataManager.deleteLens(lens.id);

      assert.strictEqual(result.success, false);
      assert.ok(result.message.includes('used in existing setups'));
    });
  });
});
