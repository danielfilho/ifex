import fs from 'fs';
import path from 'path';
import os from 'os';

const CONFIG_DIR = path.join(os.homedir(), '.config');
const CONFIG_FILE = path.join(CONFIG_DIR, 'ifex.json');

export class ConfigManager {
  constructor() {
    this.ensureConfigDir();
  }

  ensureConfigDir() {
    if (!fs.existsSync(CONFIG_DIR)) {
      fs.mkdirSync(CONFIG_DIR, { recursive: true });
    }
  }

  loadConfig() {
    if (!fs.existsSync(CONFIG_FILE)) {
      return [];
    }

    try {
      const data = fs.readFileSync(CONFIG_FILE, 'utf8');
      return JSON.parse(data);
    } catch (error) {
      console.error('Error reading config file:', error.message);
      return [];
    }
  }

  saveConfig(configs) {
    try {
      fs.writeFileSync(CONFIG_FILE, JSON.stringify(configs, null, 2));
      return true;
    } catch (error) {
      console.error('Error saving config file:', error.message);
      return false;
    }
  }

  addConfig(config) {
    const configs = this.loadConfig();
    config.id = Date.now().toString();
    config.createdAt = new Date().toISOString();
    configs.push(config);
    return this.saveConfig(configs);
  }

  getConfigs() {
    return this.loadConfig();
  }

  deleteConfig(id) {
    const configs = this.loadConfig();
    const filtered = configs.filter(config => config.id !== id);
    return this.saveConfig(filtered);
  }

  getConfigById(id) {
    const configs = this.loadConfig();
    return configs.find(config => config.id === id);
  }
}
