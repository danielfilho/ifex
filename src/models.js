import { v4 as uuidv4 } from 'uuid';

export class Camera {
  constructor(maker, model) {
    this.id = uuidv4();
    this.maker = maker;
    this.model = model;
    this.createdAt = new Date().toISOString();
  }

  static fromObject(obj) {
    const camera = new Camera(obj.maker, obj.model);
    camera.id = obj.id;
    camera.createdAt = obj.createdAt;
    return camera;
  }

  getDisplayName() {
    return `${this.maker} ${this.model}`;
  }
}

export class Lens {
  constructor(maker, model, focalLength, aperture, mount) {
    this.id = uuidv4();
    this.maker = maker;
    this.model = model;
    this.focalLength = focalLength;
    this.aperture = aperture;
    this.mount = mount;
    this.createdAt = new Date().toISOString();
  }

  static fromObject(obj) {
    const lens = new Lens(obj.maker, obj.model, obj.focalLength, obj.aperture, obj.mount);
    lens.id = obj.id;
    lens.createdAt = obj.createdAt;
    return lens;
  }

  getDisplayName() {
    const parts = [this.maker];

    // Build the lens description
    if (this.focalLength) {
      parts.push(`${this.focalLength}mm`);
    } else if (this.model) {
      parts.push(this.model);
    }

    if (this.aperture) parts.push(`f/${this.aperture}`);
    if (this.mount) parts.push(`(${this.mount})`);
    return parts.join(' ');
  }

  getLensModelWithAperture() {
    const parts = [];

    // Always include the lens model name if available
    if (this.model) {
      parts.push(this.model);
    }

    // Add focal length if available
    if (this.focalLength) {
      parts.push(`${this.focalLength}mm`);
    }

    // Add aperture if available
    if (this.aperture) {
      parts.push(`f/${this.aperture}`);
    }

    return parts.join(' ');
  }
}

export class Film {
  constructor(maker, name, iso) {
    this.id = uuidv4();
    this.maker = maker;
    this.name = name;
    this.iso = iso;
    this.createdAt = new Date().toISOString();
  }

  static fromObject(obj) {
    const film = new Film(obj.maker, obj.name, obj.iso);
    film.id = obj.id;
    film.createdAt = obj.createdAt;
    return film;
  }

  getDisplayName() {
    return `${this.maker} ${this.name} (ISO ${this.iso})`;
  }
}

export class Photographer {
  constructor(name, email = null) {
    this.id = uuidv4();
    this.name = name;
    this.email = email;
    this.createdAt = new Date().toISOString();
  }

  static fromObject(obj) {
    const photographer = new Photographer(obj.name, obj.email);
    photographer.id = obj.id;
    photographer.createdAt = obj.createdAt;
    return photographer;
  }

  getDisplayName() {
    return this.email ? `${this.name} (${this.email})` : this.name;
  }
}

export class Setup {
  constructor(name, cameraId, lensId) {
    this.id = uuidv4();
    this.name = name;
    this.cameraId = cameraId;
    this.lensId = lensId;
    this.createdAt = new Date().toISOString();
  }

  static fromObject(obj) {
    const setup = new Setup(obj.name, obj.cameraId, obj.lensId);
    setup.id = obj.id;
    setup.createdAt = obj.createdAt;
    return setup;
  }

  getDisplayName() {
    return this.name;
  }
}
