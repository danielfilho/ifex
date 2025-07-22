import fs from 'fs';
import path from 'path';
import os from 'os';
import { Camera, Lens, Film, Photographer, Setup } from './models.js';

const CONFIG_DIR = path.join(os.homedir(), '.config');
const DEFAULT_DATA_FILE = path.join(CONFIG_DIR, 'ifex.json');

export class DataManager {
  constructor(dataFile = null) {
    this.DATA_FILE = dataFile || DEFAULT_DATA_FILE;
    this.CONFIG_DIR = path.dirname(this.DATA_FILE);
    this.ensureConfigDir();
    this.data = this.loadData();
  }

  ensureConfigDir() {
    if (!fs.existsSync(this.CONFIG_DIR)) {
      fs.mkdirSync(this.CONFIG_DIR, { recursive: true });
    }
  }

  loadData() {
    if (!fs.existsSync(this.DATA_FILE)) {
      return {
        cameras: [],
        lenses: [],
        films: [],
        photographers: [],
        setups: []
      };
    }

    try {
      const data = fs.readFileSync(this.DATA_FILE, 'utf8');
      const parsed = JSON.parse(data);

      return {
        cameras: parsed.cameras || [],
        lenses: parsed.lenses || [],
        films: parsed.films || [],
        photographers: parsed.photographers || [],
        setups: parsed.setups || []
      };
    } catch (error) {
      console.error('Error reading data file:', error.message);
      return {
        cameras: [],
        lenses: [],
        films: [],
        photographers: [],
        setups: []
      };
    }
  }

  saveData() {
    try {
      fs.writeFileSync(this.DATA_FILE, JSON.stringify(this.data, null, 2));
      return true;
    } catch (error) {
      console.error('Error saving data file:', error.message);
      return false;
    }
  }

  // Camera CRUD operations
  createCamera(maker, model) {
    const camera = new Camera(maker, model);
    this.data.cameras.push(camera);
    return this.saveData() ? camera : null;
  }

  getCameras() {
    return this.data.cameras.map(c => Camera.fromObject(c));
  }

  getCameraById(id) {
    const camera = this.data.cameras.find(c => c.id === id);
    return camera ? Camera.fromObject(camera) : null;
  }

  updateCamera(id, maker, model) {
    const index = this.data.cameras.findIndex(c => c.id === id);
    if (index === -1) return false;

    this.data.cameras[index].maker = maker;
    this.data.cameras[index].model = model;
    return this.saveData();
  }

  deleteCamera(id) {
    const setupsUsingCamera = this.data.setups.filter(s => s.cameraId === id);
    if (setupsUsingCamera.length > 0) {
      return { success: false, message: 'Cannot delete camera: it is used in existing setups' };
    }

    this.data.cameras = this.data.cameras.filter(c => c.id !== id);
    return { success: this.saveData(), message: 'Camera deleted successfully' };
  }

  // Lens CRUD operations
  createLens(maker, model, focalLength, aperture, mount) {
    const lens = new Lens(maker, model, focalLength, aperture, mount);
    this.data.lenses.push(lens);
    return this.saveData() ? lens : null;
  }

  getLenses() {
    return this.data.lenses.map(l => Lens.fromObject(l));
  }

  getLensById(id) {
    const lens = this.data.lenses.find(l => l.id === id);
    return lens ? Lens.fromObject(lens) : null;
  }

  updateLens(id, maker, model, focalLength, aperture, mount) {
    const index = this.data.lenses.findIndex(l => l.id === id);
    if (index === -1) return false;

    this.data.lenses[index].maker = maker;
    this.data.lenses[index].model = model;
    this.data.lenses[index].focalLength = focalLength;
    this.data.lenses[index].aperture = aperture;
    this.data.lenses[index].mount = mount;
    return this.saveData();
  }

  deleteLens(id) {
    const setupsUsingLens = this.data.setups.filter(s => s.lensId === id);
    if (setupsUsingLens.length > 0) {
      return { success: false, message: 'Cannot delete lens: it is used in existing setups' };
    }

    this.data.lenses = this.data.lenses.filter(l => l.id !== id);
    return { success: this.saveData(), message: 'Lens deleted successfully' };
  }

  // Film CRUD operations
  createFilm(maker, name, iso) {
    const film = new Film(maker, name, iso);
    this.data.films.push(film);
    return this.saveData() ? film : null;
  }

  getFilms() {
    return this.data.films.map(f => Film.fromObject(f));
  }

  getFilmById(id) {
    const film = this.data.films.find(f => f.id === id);
    return film ? Film.fromObject(film) : null;
  }

  updateFilm(id, maker, name, iso) {
    const index = this.data.films.findIndex(f => f.id === id);
    if (index === -1) return false;

    this.data.films[index].maker = maker;
    this.data.films[index].name = name;
    this.data.films[index].iso = iso;
    return this.saveData();
  }

  deleteFilm(id) {
    this.data.films = this.data.films.filter(f => f.id !== id);
    return { success: this.saveData(), message: 'Film deleted successfully' };
  }

  // Photographer CRUD operations
  createPhotographer(name, email = null) {
    const photographer = new Photographer(name, email);
    this.data.photographers.push(photographer);
    this.saveData();
    return photographer;
  }

  getPhotographers() {
    return this.data.photographers.map(p => Photographer.fromObject(p));
  }

  getPhotographerById(id) {
    const photographer = this.data.photographers.find(p => p.id === id);
    return photographer ? Photographer.fromObject(photographer) : null;
  }

  updatePhotographer(id, name, email = null) {
    const index = this.data.photographers.findIndex(p => p.id === id);
    if (index === -1) return false;

    this.data.photographers[index].name = name;
    this.data.photographers[index].email = email;
    return this.saveData();
  }

  deletePhotographer(id) {
    this.data.photographers = this.data.photographers.filter(p => p.id !== id);
    return { success: this.saveData(), message: 'Photographer deleted successfully' };
  }

  // Setup CRUD operations
  createSetup(name, cameraId, lensId) {
    const camera = this.getCameraById(cameraId);
    const lens = this.getLensById(lensId);

    if (!camera || !lens) {
      return null;
    }

    const setup = new Setup(name, cameraId, lensId);
    this.data.setups.push(setup);
    return this.saveData() ? setup : null;
  }

  getSetups() {
    return this.data.setups.map(s => Setup.fromObject(s));
  }

  getSetupById(id) {
    const setup = this.data.setups.find(s => s.id === id);
    return setup ? Setup.fromObject(setup) : null;
  }

  getSetupWithDetails(id) {
    const setup = this.getSetupById(id);
    if (!setup) return null;

    const camera = this.getCameraById(setup.cameraId);
    const lens = this.getLensById(setup.lensId);

    return {
      setup,
      camera,
      lens
    };
  }

  updateSetup(id, name, cameraId, lensId) {
    const index = this.data.setups.findIndex(s => s.id === id);
    if (index === -1) return false;

    const camera = this.getCameraById(cameraId);
    const lens = this.getLensById(lensId);

    if (!camera || !lens) {
      return false;
    }

    this.data.setups[index].name = name;
    this.data.setups[index].cameraId = cameraId;
    this.data.setups[index].lensId = lensId;
    return this.saveData();
  }

  deleteSetup(id) {
    this.data.setups = this.data.setups.filter(s => s.id !== id);
    return { success: this.saveData(), message: 'Setup deleted successfully' };
  }
}
