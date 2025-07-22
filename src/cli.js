import inquirer from 'inquirer';
import chalk from 'chalk';
import { DataManager } from './dataManager.js';
import { ManagementCLI } from './managementCLI.js';
import { promptWithCancel, wrapWithCancelHandler } from './promptUtils.js';
import { createFuzzySearchPromptWithInstructions } from './searchUtils.js';

export class CLI {
  constructor() {
    this.dataManager = new DataManager();
    this.managementCLI = new ManagementCLI(this.dataManager);
  }

  async promptMainMenu() {
    const { choice } = await inquirer.prompt([
      {
        type: 'list',
        name: 'choice',
        message: 'What would you like to do?',
        choices: [
          { name: '📸 Apply EXIF data to images files', value: 'apply' },
          { name: '🗑️  Erase all EXIF data from images files', value: 'erase' },
          { name: '⚙️  Manage Equipment & Setups', value: 'manage' },
          { name: '🚪 Exit', value: 'exit' }
        ]
      }
    ]);

    return choice;
  }

  selectSetupAndFilm = wrapWithCancelHandler(async function() {
    const setups = this.dataManager.getSetups();
    const films = this.dataManager.getFilms();
    const photographers = this.dataManager.getPhotographers();

    if (setups.length === 0) {
      console.log(chalk.yellow('⚠️  No setups available. Please create a setup first.'));
      return null;
    }

    if (films.length === 0) {
      console.log(chalk.yellow('⚠️  No films available. Please add films first.'));
      return null;
    }

    if (photographers.length === 0) {
      console.log(chalk.yellow('⚠️  No photographers available. Please add photographers first.'));
      return null;
    }

    const setupChoices = setups.map(setup => {
      const details = this.dataManager.getSetupWithDetails(setup.id);
      return {
        name: `${setup.name} (${details.camera.getDisplayName()} + ${details.lens.getDisplayName()})`,
        value: setup.id
      };
    });

    const filmChoices = films.map(film => ({
      name: film.getDisplayName(),
      value: film.id
    }));

    const photographerChoices = photographers.map(photographer => ({
      name: photographer.getDisplayName(),
      value: photographer.id
    }));

    const selection = await promptWithCancel([
      createFuzzySearchPromptWithInstructions(
        'setupId',
        'Select a setup (Camera + Lens)',
        setupChoices,
        { pageSize: 10 }
      ),
      createFuzzySearchPromptWithInstructions(
        'filmId',
        'Select a film',
        filmChoices,
        { pageSize: 10 }
      ),
      {
        type: 'input',
        name: 'shotISO',
        message: 'ISO Shot (leave empty to use film ISO):',
        validate: input => {
          if (!input.trim()) return true;
          const iso = parseInt(input);
          return !isNaN(iso) && iso > 0 ? true : 'Please enter a valid ISO number';
        }
      },
      createFuzzySearchPromptWithInstructions(
        'photographerId',
        'Select photographer',
        photographerChoices,
        { pageSize: 10 }
      )
    ]);

    const setupDetails = this.dataManager.getSetupWithDetails(selection.setupId);
    const film = this.dataManager.getFilmById(selection.filmId);
    const photographer = this.dataManager.getPhotographerById(selection.photographerId);

    return {
      setup: setupDetails,
      film,
      shotISO: selection.shotISO ? parseInt(selection.shotISO) : film.iso,
      photographer: photographer.name
    };
  });

  promptFolderPath = wrapWithCancelHandler(async function() {
    const { folderPath } = await promptWithCancel([
      {
        type: 'input',
        name: 'folderPath',
        message: 'Enter the folder path containing image files:',
        validate: input => input.trim() ? true : 'Please enter a folder path'
      }
    ]);

    return this.cleanFolderPath(folderPath);
  });

  cleanFolderPath(inputPath) {
    let cleanPath = inputPath.trim();

    // Remove surrounding quotes if present and trim again
    if ((cleanPath.startsWith('"') && cleanPath.endsWith('"')) ||
        (cleanPath.startsWith("'") && cleanPath.endsWith("'"))) {
      cleanPath = cleanPath.slice(1, -1).trim();
    }

    // Handle escaped spaces - convert "\ " back to " "
    cleanPath = cleanPath.replace(/\\ /g, ' ');

    return cleanPath;
  }

  async confirmEraseExif() {
    const { confirm } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'confirm',
        message: chalk.red('⚠️  This will permanently remove ALL EXIF data from the image files. Are you sure?'),
        default: false
      }
    ]);

    return confirm;
  }

  async promptRecursiveProcessing() {
    const { recursive } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'recursive',
        message: 'Process subdirectories recursively?',
        default: true
      }
    ]);

    return recursive;
  }

  displaySelection(selection) {
    console.log(chalk.blue('\n📋 EXIF Configuration:'));
    console.log(`Setup: ${selection.setup.setup.name}`);
    console.log(`Camera: ${selection.setup.camera.getDisplayName()}`);
    console.log(`Lens: ${selection.setup.lens.getDisplayName()}`);
    console.log(`Film: ${selection.film.getDisplayName()}`);
    console.log(`Shot ISO: ${selection.shotISO}`);
    if (selection.photographer) console.log(`Photographer: ${selection.photographer}`);
    console.log('');
  }
}
