import { test, describe } from 'node:test';
import assert from 'node:assert';
import { Camera, Lens, Film, Photographer, Setup } from '../src/models.js';

describe('Camera Model', () => {
  test('should create a camera with maker and model', () => {
    const camera = new Camera('Canon', 'AE-1');

    assert.strictEqual(camera.maker, 'Canon');
    assert.strictEqual(camera.model, 'AE-1');
    assert.ok(camera.id);
    assert.ok(camera.createdAt);
  });

  test('should create camera from object', () => {
    const obj = {
      id: 'test-id',
      maker: 'Nikon',
      model: 'FM2',
      createdAt: '2023-01-01T00:00:00.000Z'
    };

    const camera = Camera.fromObject(obj);

    assert.strictEqual(camera.id, 'test-id');
    assert.strictEqual(camera.maker, 'Nikon');
    assert.strictEqual(camera.model, 'FM2');
    assert.strictEqual(camera.createdAt, '2023-01-01T00:00:00.000Z');
  });

  test('should generate display name', () => {
    const camera = new Camera('Pentax', 'K1000');

    assert.strictEqual(camera.getDisplayName(), 'Pentax K1000');
  });
});

describe('Lens Model', () => {
  test('should create a lens with all fields', () => {
    const lens = new Lens('Canon', 'FD', '50', '1.4', 'FD');

    assert.strictEqual(lens.maker, 'Canon');
    assert.strictEqual(lens.model, 'FD');
    assert.strictEqual(lens.focalLength, '50');
    assert.strictEqual(lens.aperture, '1.4');
    assert.strictEqual(lens.mount, 'FD');
    assert.ok(lens.id);
    assert.ok(lens.createdAt);
  });

  test('should create lens from object', () => {
    const obj = {
      id: 'lens-test-id',
      maker: 'Nikon',
      model: 'AI',
      focalLength: '35',
      aperture: '2.8',
      mount: 'F',
      createdAt: '2023-01-01T00:00:00.000Z'
    };

    const lens = Lens.fromObject(obj);

    assert.strictEqual(lens.id, 'lens-test-id');
    assert.strictEqual(lens.maker, 'Nikon');
    assert.strictEqual(lens.focalLength, '35');
  });

  test('should generate display name with all fields', () => {
    const lens = new Lens('Canon', 'FD', '50', '1.2', 'FD');

    assert.strictEqual(lens.getDisplayName(), 'Canon 50mm f/1.2 (FD)');
  });

  test('should generate display name with focal length only', () => {
    const lens = new Lens('Nikon', null, '85', '1.8', 'F');

    assert.strictEqual(lens.getDisplayName(), 'Nikon 85mm f/1.8 (F)');
  });

  test('should generate display name with model only when no focal length', () => {
    const lens = new Lens('Canon', 'Macro', null, '2.8', 'FD');

    assert.strictEqual(lens.getDisplayName(), 'Canon Macro f/2.8 (FD)');
  });

  test('should generate lens model with aperture for EXIF', () => {
    const lens = new Lens('Canon', 'FD', '50', '1.2', 'FD');

    assert.strictEqual(lens.getLensModelWithAperture(), '50mm f/1.2');
  });

  test('should handle zoom lens focal length', () => {
    const lens = new Lens('Pentax', 'SMC', '28-135', '4', 'K');

    assert.strictEqual(lens.getDisplayName(), 'Pentax 28-135mm f/4 (K)');
    assert.strictEqual(lens.getLensModelWithAperture(), '28-135mm f/4');
  });
});

describe('Film Model', () => {
  test('should create a film with maker, name, and ISO', () => {
    const film = new Film('Kodak', 'Portra', 400);

    assert.strictEqual(film.maker, 'Kodak');
    assert.strictEqual(film.name, 'Portra');
    assert.strictEqual(film.iso, 400);
    assert.ok(film.id);
    assert.ok(film.createdAt);
  });

  test('should create film from object', () => {
    const obj = {
      id: 'film-test-id',
      maker: 'Fuji',
      name: 'Velvia',
      iso: 50,
      createdAt: '2023-01-01T00:00:00.000Z'
    };

    const film = Film.fromObject(obj);

    assert.strictEqual(film.id, 'film-test-id');
    assert.strictEqual(film.maker, 'Fuji');
    assert.strictEqual(film.name, 'Velvia');
    assert.strictEqual(film.iso, 50);
  });

  test('should generate display name', () => {
    const film = new Film('Kodak', 'Gold', 200);

    assert.strictEqual(film.getDisplayName(), 'Kodak Gold (ISO 200)');
  });
});

describe('Photographer Model', () => {
  test('should create a photographer with name only', () => {
    const photographer = new Photographer('John Doe');

    assert.strictEqual(photographer.name, 'John Doe');
    assert.strictEqual(photographer.email, null);
    assert.ok(photographer.id);
    assert.ok(photographer.createdAt);
  });

  test('should create a photographer with name and email', () => {
    const photographer = new Photographer('Jane Smith', 'jane@example.com');

    assert.strictEqual(photographer.name, 'Jane Smith');
    assert.strictEqual(photographer.email, 'jane@example.com');
    assert.ok(photographer.id);
    assert.ok(photographer.createdAt);
  });

  test('should create photographer from object', () => {
    const obj = {
      id: 'photographer-test-id',
      name: 'Bob Wilson',
      email: 'bob@photography.com',
      createdAt: '2023-01-01T00:00:00.000Z'
    };

    const photographer = Photographer.fromObject(obj);

    assert.strictEqual(photographer.id, 'photographer-test-id');
    assert.strictEqual(photographer.name, 'Bob Wilson');
    assert.strictEqual(photographer.email, 'bob@photography.com');
    assert.strictEqual(photographer.createdAt, '2023-01-01T00:00:00.000Z');
  });

  test('should generate display name with email', () => {
    const photographer = new Photographer('Alice Brown', 'alice@studio.com');

    assert.strictEqual(photographer.getDisplayName(), 'Alice Brown (alice@studio.com)');
  });

  test('should generate display name without email', () => {
    const photographer = new Photographer('Charlie Davis');

    assert.strictEqual(photographer.getDisplayName(), 'Charlie Davis');
  });
});

describe('Setup Model', () => {
  test('should create a setup with name and IDs', () => {
    const setup = new Setup('Street Photography', 'camera-id', 'lens-id');

    assert.strictEqual(setup.name, 'Street Photography');
    assert.strictEqual(setup.cameraId, 'camera-id');
    assert.strictEqual(setup.lensId, 'lens-id');
    assert.ok(setup.id);
    assert.ok(setup.createdAt);
  });

  test('should create setup from object', () => {
    const obj = {
      id: 'setup-test-id',
      name: 'Studio Portraits',
      cameraId: 'camera-123',
      lensId: 'lens-456',
      createdAt: '2023-01-01T00:00:00.000Z'
    };

    const setup = Setup.fromObject(obj);

    assert.strictEqual(setup.id, 'setup-test-id');
    assert.strictEqual(setup.name, 'Studio Portraits');
    assert.strictEqual(setup.cameraId, 'camera-123');
    assert.strictEqual(setup.lensId, 'lens-456');
  });

  test('should generate display name', () => {
    const setup = new Setup('Landscape Photography', 'cam-1', 'lens-1');

    assert.strictEqual(setup.getDisplayName(), 'Landscape Photography');
  });
});
