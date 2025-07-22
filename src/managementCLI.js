import inquirer from 'inquirer';
import chalk from 'chalk';
import { promptWithCancel, wrapWithCancelHandler } from './promptUtils.js';
import { createFuzzySearchPromptWithInstructions } from './searchUtils.js';

export class ManagementCLI {
  constructor(dataManager) {
    this.dataManager = dataManager;
  }

  async showMainManagementMenu() {
    const { choice } = await inquirer.prompt([
      {
        type: 'list',
        name: 'choice',
        message: 'What would you like to manage?',
        choices: [
          { name: '📷 Manage Cameras', value: 'cameras' },
          { name: '🔍 Manage Lenses', value: 'lenses' },
          { name: '🎞️  Manage Films', value: 'films' },
          { name: '👤 Manage Photographers', value: 'photographers' },
          { name: '⚙️  Manage Setups (Camera + Lens)', value: 'setups' },
          { name: '← Back to Main Menu', value: 'back' }
        ]
      }
    ]);

    switch (choice) {
      case 'cameras':
        return await this.manageCameras();
      case 'lenses':
        return await this.manageLenses();
      case 'films':
        return await this.manageFilms();
      case 'photographers':
        return await this.managePhotographers();
      case 'setups':
        return await this.manageSetups();
      case 'back':
        return 'back';
      default:
        return null;
    }
  }

  // Camera Management
  async manageCameras() {
    while (true) {
      const cameras = this.dataManager.getCameras();

      if (cameras.length === 0) {
        const { choice } = await inquirer.prompt([
          {
            type: 'list',
            name: 'choice',
            message: 'Camera Management (no cameras found):',
            choices: [
              { name: '➕ Add New Camera', value: 'add' },
              { name: '← Back', value: 'back' }
            ]
          }
        ]);

        if (choice === 'add') {
          await this.addCamera();
        } else if (choice === 'back') {
          break;
        }
        continue;
      }

      const choices = [
        { name: '➕ Add New Camera', value: 'add' },
        ...cameras.map(camera => ({
          name: `${camera.getDisplayName()} (${new Date(camera.createdAt).toLocaleDateString()})`,
          value: `camera_${camera.id}`
        })),
        { name: '← Back', value: 'back' }
      ];

      const { choice } = await inquirer.prompt([
        createFuzzySearchPromptWithInstructions(
          'choice',
          'Camera Management',
          choices,
          { pageSize: 15 }
        )
      ]);

      if (choice === 'add') {
        await this.addCamera();
      } else if (choice === 'back') {
        break;
      } else if (choice.startsWith('camera_')) {
        const cameraId = choice.replace('camera_', '');
        await this.manageSingleCamera(cameraId);
      }
    }
  }

  addCamera = wrapWithCancelHandler(async function() {
    const camera = await promptWithCancel([
      {
        type: 'input',
        name: 'maker',
        message: 'Camera Maker:',
        validate: input => input.trim() ? true : 'Please enter a camera maker'
      },
      {
        type: 'input',
        name: 'model',
        message: 'Camera Model:',
        validate: input => input.trim() ? true : 'Please enter a camera model'
      }
    ]);

    const result = this.dataManager.createCamera(camera.maker, camera.model);
    if (result) {
      console.log(chalk.green('✅ Camera added successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to add camera'));
    }
  });

  async manageSingleCamera(cameraId) {
    const camera = this.dataManager.getCameraById(cameraId);
    if (!camera) {
      console.log(chalk.red('Camera not found'));
      return;
    }

    const { action } = await inquirer.prompt([
      {
        type: 'list',
        name: 'action',
        message: `Manage ${camera.getDisplayName()}:`,
        choices: [
          { name: '✏️  Edit', value: 'edit' },
          { name: '🗑️  Delete', value: 'delete' },
          { name: '← Back', value: 'back' }
        ]
      }
    ]);

    switch (action) {
      case 'edit':
        await this.editCamera(cameraId);
        break;
      case 'delete':
        await this.deleteCamera(cameraId);
        break;
    }
  }

  editCamera = wrapWithCancelHandler(async function(cameraId) {
    const camera = this.dataManager.getCameraById(cameraId);
    const updated = await promptWithCancel([
      {
        type: 'input',
        name: 'maker',
        message: 'Camera Maker:',
        default: camera.maker,
        validate: input => input.trim() ? true : 'Please enter a camera maker'
      },
      {
        type: 'input',
        name: 'model',
        message: 'Camera Model:',
        default: camera.model,
        validate: input => input.trim() ? true : 'Please enter a camera model'
      }
    ]);

    if (this.dataManager.updateCamera(cameraId, updated.maker, updated.model)) {
      console.log(chalk.green('✅ Camera updated successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to update camera'));
    }
  });

  async deleteCamera(cameraId) {
    const camera = this.dataManager.getCameraById(cameraId);
    const { confirm } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'confirm',
        message: `Are you sure you want to delete ${camera.getDisplayName()}?`,
        default: false
      }
    ]);

    if (confirm) {
      const result = this.dataManager.deleteCamera(cameraId);
      if (result.success) {
        console.log(chalk.green('✅ Camera deleted successfully!'));
      } else {
        console.log(chalk.red(`❌ ${result.message}`));
      }
    }
  }

  // Lens Management
  async manageLenses() {
    while (true) {
      const lenses = this.dataManager.getLenses();

      if (lenses.length === 0) {
        const { choice } = await inquirer.prompt([
          {
            type: 'list',
            name: 'choice',
            message: 'Lens Management (no lenses found):',
            choices: [
              { name: '➕ Add New Lens', value: 'add' },
              { name: '← Back', value: 'back' }
            ]
          }
        ]);

        if (choice === 'add') {
          await this.addLens();
        } else if (choice === 'back') {
          break;
        }
        continue;
      }

      const choices = [
        { name: '➕ Add New Lens', value: 'add' },
        ...lenses.map(lens => ({
          name: `${lens.getDisplayName()} (${new Date(lens.createdAt).toLocaleDateString()})`,
          value: `lens_${lens.id}`
        })),
        { name: '← Back', value: 'back' }
      ];

      const { choice } = await inquirer.prompt([
        createFuzzySearchPromptWithInstructions(
          'choice',
          'Lens Management',
          choices,
          { pageSize: 15 }
        )
      ]);

      if (choice === 'add') {
        await this.addLens();
      } else if (choice === 'back') {
        break;
      } else if (choice.startsWith('lens_')) {
        const lensId = choice.replace('lens_', '');
        await this.manageSingleLens(lensId);
      }
    }
  }

  addLens = wrapWithCancelHandler(async function() {
    const lens = await promptWithCancel([
      {
        type: 'input',
        name: 'maker',
        message: 'Lens Maker:',
        validate: input => input.trim() ? true : 'Please enter a lens maker'
      },
      {
        type: 'input',
        name: 'model',
        message: 'Lens Model:',
        validate: _input => true // Optional if focal length is provided
      },
      {
        type: 'input',
        name: 'focalLength',
        message: 'Focal Length (mm, e.g., 50, 24-70):',
        validate: input => {
          if (!input.trim()) return true; // Optional field
          // Allow ranges like "24-70" or single values like "50"
          return /^(\d+(-\d+)?)$/.test(input.trim()) ? true : 'Please enter a valid focal length (e.g., 50 or 24-70)';
        }
      },
      {
        type: 'input',
        name: 'aperture',
        message: 'Maximum Aperture (e.g., 1.4, 2.8):',
        validate: input => {
          if (!input.trim()) return true; // Optional field
          const aperture = parseFloat(input);
          return !isNaN(aperture) && aperture > 0 ? true : 'Please enter a valid aperture value';
        }
      },
      {
        type: 'input',
        name: 'mount',
        message: 'Lens Mount (e.g., FD, F, K):',
        validate: input => input.trim() ? true : 'Please enter a lens mount'
      }
    ]);

    // Validate that at least model or focal length is provided
    if (!lens.model.trim() && !lens.focalLength.trim()) {
      console.log(chalk.red('❌ Please provide either a lens model or focal length'));
      return;
    }

    const result = this.dataManager.createLens(
      lens.maker,
      lens.model || null,
      lens.focalLength || null,
      lens.aperture || null,
      lens.mount
    );
    if (result) {
      console.log(chalk.green('✅ Lens added successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to add lens'));
    }
  });

  async manageSingleLens(lensId) {
    const lens = this.dataManager.getLensById(lensId);
    if (!lens) {
      console.log(chalk.red('Lens not found'));
      return;
    }

    const { action } = await inquirer.prompt([
      {
        type: 'list',
        name: 'action',
        message: `Manage ${lens.getDisplayName()}:`,
        choices: [
          { name: '✏️  Edit', value: 'edit' },
          { name: '🗑️  Delete', value: 'delete' },
          { name: '← Back', value: 'back' }
        ]
      }
    ]);

    switch (action) {
      case 'edit':
        await this.editLens(lensId);
        break;
      case 'delete':
        await this.deleteLens(lensId);
        break;
    }
  }

  async editLens(lensId) {
    const lens = this.dataManager.getLensById(lensId);
    console.log(chalk.gray('Press Ctrl+C to cancel and go back'));

    const updated = await inquirer.prompt([
      {
        type: 'input',
        name: 'maker',
        message: 'Lens Maker:',
        default: lens.maker,
        validate: input => input.trim() ? true : 'Please enter a lens maker'
      },
      {
        type: 'input',
        name: 'model',
        message: 'Lens Model:',
        default: lens.model || '',
        validate: _input => true // Optional if focal length is provided
      },
      {
        type: 'input',
        name: 'focalLength',
        message: 'Focal Length (mm, e.g., 50, 24-70):',
        default: lens.focalLength || '',
        validate: input => {
          if (!input.trim()) return true; // Optional field
          // Allow ranges like "24-70" or single values like "50"
          return /^(\d+(-\d+)?)$/.test(input.trim()) ? true : 'Please enter a valid focal length (e.g., 50 or 24-70)';
        }
      },
      {
        type: 'input',
        name: 'aperture',
        message: 'Maximum Aperture (e.g., 1.4, 2.8):',
        default: lens.aperture || '',
        validate: input => {
          if (!input.trim()) return true; // Optional field
          const aperture = parseFloat(input);
          return !isNaN(aperture) && aperture > 0 ? true : 'Please enter a valid aperture value';
        }
      },
      {
        type: 'input',
        name: 'mount',
        message: 'Lens Mount (e.g., FD, F, K):',
        default: lens.mount || '',
        validate: input => input.trim() ? true : 'Please enter a lens mount'
      }
    ]);

    // Validate that at least model or focal length is provided
    if (!updated.model.trim() && !updated.focalLength.trim()) {
      console.log(chalk.red('❌ Please provide either a lens model or focal length'));
      return;
    }

    if (this.dataManager.updateLens(
      lensId,
      updated.maker,
      updated.model || null,
      updated.focalLength || null,
      updated.aperture || null,
      updated.mount
    )) {
      console.log(chalk.green('✅ Lens updated successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to update lens'));
    }
  }

  async deleteLens(lensId) {
    const lens = this.dataManager.getLensById(lensId);
    const { confirm } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'confirm',
        message: `Are you sure you want to delete ${lens.getDisplayName()}?`,
        default: false
      }
    ]);

    if (confirm) {
      const result = this.dataManager.deleteLens(lensId);
      if (result.success) {
        console.log(chalk.green('✅ Lens deleted successfully!'));
      } else {
        console.log(chalk.red(`❌ ${result.message}`));
      }
    }
  }

  // Film Management
  async manageFilms() {
    while (true) {
      const films = this.dataManager.getFilms();

      if (films.length === 0) {
        const { choice } = await inquirer.prompt([
          {
            type: 'list',
            name: 'choice',
            message: 'Film Management (no films found):',
            choices: [
              { name: '➕ Add New Film', value: 'add' },
              { name: '← Back', value: 'back' }
            ]
          }
        ]);

        if (choice === 'add') {
          await this.addFilm();
        } else if (choice === 'back') {
          break;
        }
        continue;
      }

      const choices = [
        { name: '➕ Add New Film', value: 'add' },
        ...films.map(film => ({
          name: `${film.getDisplayName()} (${new Date(film.createdAt).toLocaleDateString()})`,
          value: `film_${film.id}`
        })),
        { name: '← Back', value: 'back' }
      ];

      const { choice } = await inquirer.prompt([
        createFuzzySearchPromptWithInstructions(
          'choice',
          'Film Management',
          choices,
          { pageSize: 15 }
        )
      ]);

      if (choice === 'add') {
        await this.addFilm();
      } else if (choice === 'back') {
        break;
      } else if (choice.startsWith('film_')) {
        const filmId = choice.replace('film_', '');
        await this.manageSingleFilm(filmId);
      }
    }
  }

  async addFilm() {
    const film = await inquirer.prompt([
      {
        type: 'input',
        name: 'maker',
        message: 'Film Maker:',
        validate: input => input.trim() ? true : 'Please enter a film maker'
      },
      {
        type: 'input',
        name: 'name',
        message: 'Film Name:',
        validate: input => input.trim() ? true : 'Please enter a film name'
      },
      {
        type: 'input',
        name: 'iso',
        message: 'Film ISO:',
        validate: input => {
          const iso = parseInt(input);
          return !isNaN(iso) && iso > 0 ? true : 'Please enter a valid ISO number';
        }
      }
    ]);

    const result = this.dataManager.createFilm(film.maker, film.name, parseInt(film.iso));
    if (result) {
      console.log(chalk.green('✅ Film added successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to add film'));
    }
  }

  async manageSingleFilm(filmId) {
    const film = this.dataManager.getFilmById(filmId);
    if (!film) {
      console.log(chalk.red('Film not found'));
      return;
    }

    const { action } = await inquirer.prompt([
      {
        type: 'list',
        name: 'action',
        message: `Manage ${film.getDisplayName()}:`,
        choices: [
          { name: '✏️  Edit', value: 'edit' },
          { name: '🗑️  Delete', value: 'delete' },
          { name: '← Back', value: 'back' }
        ]
      }
    ]);

    switch (action) {
      case 'edit':
        await this.editFilm(filmId);
        break;
      case 'delete':
        await this.deleteFilm(filmId);
        break;
    }
  }

  async editFilm(filmId) {
    const film = this.dataManager.getFilmById(filmId);
    const updated = await inquirer.prompt([
      {
        type: 'input',
        name: 'maker',
        message: 'Film Maker:',
        default: film.maker,
        validate: input => input.trim() ? true : 'Please enter a film maker'
      },
      {
        type: 'input',
        name: 'name',
        message: 'Film Name:',
        default: film.name,
        validate: input => input.trim() ? true : 'Please enter a film name'
      },
      {
        type: 'input',
        name: 'iso',
        message: 'Film ISO:',
        default: film.iso.toString(),
        validate: input => {
          const iso = parseInt(input);
          return !isNaN(iso) && iso > 0 ? true : 'Please enter a valid ISO number';
        }
      }
    ]);

    if (this.dataManager.updateFilm(filmId, updated.maker, updated.name, parseInt(updated.iso))) {
      console.log(chalk.green('✅ Film updated successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to update film'));
    }
  }

  async deleteFilm(filmId) {
    const film = this.dataManager.getFilmById(filmId);
    const { confirm } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'confirm',
        message: `Are you sure you want to delete ${film.getDisplayName()}?`,
        default: false
      }
    ]);

    if (confirm) {
      const result = this.dataManager.deleteFilm(filmId);
      if (result.success) {
        console.log(chalk.green('✅ Film deleted successfully!'));
      } else {
        console.log(chalk.red(`❌ ${result.message}`));
      }
    }
  }

  // Photographer Management
  async managePhotographers() {
    while (true) {
      const photographers = this.dataManager.getPhotographers();

      if (photographers.length === 0) {
        const { choice } = await inquirer.prompt([
          {
            type: 'list',
            name: 'choice',
            message: 'Photographer Management (no photographers found):',
            choices: [
              { name: '➕ Add New Photographer', value: 'add' },
              { name: '← Back', value: 'back' }
            ]
          }
        ]);

        if (choice === 'add') {
          await this.addPhotographer();
        } else if (choice === 'back') {
          break;
        }
        continue;
      }

      const choices = [
        { name: '➕ Add New Photographer', value: 'add' },
        ...photographers.map(photographer => ({
          name: `${photographer.getDisplayName()} (${new Date(photographer.createdAt).toLocaleDateString()})`,
          value: `photographer_${photographer.id}`
        })),
        { name: '← Back', value: 'back' }
      ];

      const { choice } = await inquirer.prompt([
        createFuzzySearchPromptWithInstructions(
          'choice',
          'Photographer Management',
          choices,
          { pageSize: 15 }
        )
      ]);

      if (choice === 'add') {
        await this.addPhotographer();
      } else if (choice === 'back') {
        break;
      } else if (choice.startsWith('photographer_')) {
        const photographerId = choice.replace('photographer_', '');
        await this.manageSinglePhotographer(photographerId);
      }
    }
  }

  addPhotographer = wrapWithCancelHandler(async function() {
    const photographer = await promptWithCancel([
      {
        type: 'input',
        name: 'name',
        message: 'Photographer Name:',
        validate: input => input.trim() ? true : 'Please enter a photographer name'
      },
      {
        type: 'input',
        name: 'email',
        message: 'Email (optional):',
        validate: input => {
          if (!input.trim()) return true; // Optional field
          const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
          return emailRegex.test(input.trim()) ? true : 'Please enter a valid email address';
        }
      }
    ]);

    const result = this.dataManager.createPhotographer(
      photographer.name,
      photographer.email || null
    );
    if (result) {
      console.log(chalk.green('✅ Photographer added successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to add photographer'));
    }
  });

  async manageSinglePhotographer(photographerId) {
    const photographer = this.dataManager.getPhotographerById(photographerId);
    if (!photographer) {
      console.log(chalk.red('Photographer not found'));
      return;
    }

    const { action } = await inquirer.prompt([
      {
        type: 'list',
        name: 'action',
        message: `Manage ${photographer.getDisplayName()}:`,
        choices: [
          { name: '✏️  Edit', value: 'edit' },
          { name: '🗑️  Delete', value: 'delete' },
          { name: '← Back', value: 'back' }
        ]
      }
    ]);

    switch (action) {
      case 'edit':
        await this.editPhotographer(photographerId);
        break;
      case 'delete':
        await this.deletePhotographer(photographerId);
        break;
    }
  }

  editPhotographer = wrapWithCancelHandler(async function(photographerId) {
    const photographer = this.dataManager.getPhotographerById(photographerId);
    const updated = await promptWithCancel([
      {
        type: 'input',
        name: 'name',
        message: 'Photographer Name:',
        default: photographer.name,
        validate: input => input.trim() ? true : 'Please enter a photographer name'
      },
      {
        type: 'input',
        name: 'email',
        message: 'Email (optional):',
        default: photographer.email || '',
        validate: input => {
          if (!input.trim()) return true; // Optional field
          const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
          return emailRegex.test(input.trim()) ? true : 'Please enter a valid email address';
        }
      }
    ]);

    if (this.dataManager.updatePhotographer(photographerId, updated.name, updated.email || null)) {
      console.log(chalk.green('✅ Photographer updated successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to update photographer'));
    }
  });

  async deletePhotographer(photographerId) {
    const photographer = this.dataManager.getPhotographerById(photographerId);
    const { confirm } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'confirm',
        message: `Are you sure you want to delete ${photographer.getDisplayName()}?`,
        default: false
      }
    ]);

    if (confirm) {
      const result = this.dataManager.deletePhotographer(photographerId);
      if (result.success) {
        console.log(chalk.green('✅ Photographer deleted successfully!'));
      } else {
        console.log(chalk.red(`❌ ${result.message}`));
      }
    }
  }

  // Setup Management
  async manageSetups() {
    while (true) {
      const setups = this.dataManager.getSetups();

      if (setups.length === 0) {
        const { choice } = await inquirer.prompt([
          {
            type: 'list',
            name: 'choice',
            message: 'Setup Management (no setups found):',
            choices: [
              { name: '➕ Add New Setup', value: 'add' },
              { name: '← Back', value: 'back' }
            ]
          }
        ]);

        if (choice === 'add') {
          await this.addSetup();
        } else if (choice === 'back') {
          break;
        }
        continue;
      }

      const choices = [
        { name: '➕ Add New Setup', value: 'add' },
        ...setups.map(setup => ({
          name: `${setup.getDisplayName()} (${new Date(setup.createdAt).toLocaleDateString()})`,
          value: `setup_${setup.id}`
        })),
        { name: '← Back', value: 'back' }
      ];

      const { choice } = await inquirer.prompt([
        createFuzzySearchPromptWithInstructions(
          'choice',
          'Setup Management',
          choices,
          { pageSize: 15 }
        )
      ]);

      if (choice === 'add') {
        await this.addSetup();
      } else if (choice === 'back') {
        break;
      } else if (choice.startsWith('setup_')) {
        const setupId = choice.replace('setup_', '');
        await this.manageSingleSetup(setupId);
      }
    }
  }

  async addSetup() {
    const cameras = this.dataManager.getCameras();
    const lenses = this.dataManager.getLenses();

    if (cameras.length === 0) {
      console.log(chalk.yellow('⚠️  No cameras available. Please add cameras first.'));
      return;
    }

    if (lenses.length === 0) {
      console.log(chalk.yellow('⚠️  No lenses available. Please add lenses first.'));
      return;
    }

    const cameraChoices = cameras.map(camera => ({
      name: camera.getDisplayName(),
      value: camera.id
    }));

    const lensChoices = lenses.map(lens => ({
      name: lens.getDisplayName(),
      value: lens.id
    }));

    const setup = await inquirer.prompt([
      {
        type: 'input',
        name: 'name',
        message: 'Setup Name:',
        validate: input => input.trim() ? true : 'Please enter a setup name'
      },
      createFuzzySearchPromptWithInstructions(
        'cameraId',
        'Select Camera',
        cameraChoices,
        { pageSize: 10 }
      ),
      createFuzzySearchPromptWithInstructions(
        'lensId',
        'Select Lens',
        lensChoices,
        { pageSize: 10 }
      )
    ]);

    const result = this.dataManager.createSetup(setup.name, setup.cameraId, setup.lensId);
    if (result) {
      console.log(chalk.green('✅ Setup added successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to add setup'));
    }
  }

  async manageSingleSetup(setupId) {
    const setupDetails = this.dataManager.getSetupWithDetails(setupId);
    if (!setupDetails) {
      console.log(chalk.red('Setup not found'));
      return;
    }

    const { setup, camera, lens } = setupDetails;

    console.log(chalk.blue('\nSetup Details:'));
    console.log(`Name: ${setup.name}`);
    console.log(`Camera: ${camera.getDisplayName()}`);
    console.log(`Lens: ${lens.getDisplayName()}`);

    const { action } = await inquirer.prompt([
      {
        type: 'list',
        name: 'action',
        message: `Manage ${setup.getDisplayName()}:`,
        choices: [
          { name: '✏️  Edit', value: 'edit' },
          { name: '🗑️  Delete', value: 'delete' },
          { name: '← Back', value: 'back' }
        ]
      }
    ]);

    switch (action) {
      case 'edit':
        await this.editSetup(setupId);
        break;
      case 'delete':
        await this.deleteSetup(setupId);
        break;
    }
  }

  async editSetup(setupId) {
    const setupDetails = this.dataManager.getSetupWithDetails(setupId);
    const cameras = this.dataManager.getCameras();
    const lenses = this.dataManager.getLenses();

    const updated = await inquirer.prompt([
      {
        type: 'input',
        name: 'name',
        message: 'Setup Name:',
        default: setupDetails.setup.name,
        validate: input => input.trim() ? true : 'Please enter a setup name'
      },
      {
        type: 'list',
        name: 'cameraId',
        message: 'Select Camera:',
        default: setupDetails.setup.cameraId,
        choices: cameras.map(camera => ({
          name: camera.getDisplayName(),
          value: camera.id
        }))
      },
      {
        type: 'list',
        name: 'lensId',
        message: 'Select Lens:',
        default: setupDetails.setup.lensId,
        choices: lenses.map(lens => ({
          name: lens.getDisplayName(),
          value: lens.id
        }))
      }
    ]);

    if (this.dataManager.updateSetup(setupId, updated.name, updated.cameraId, updated.lensId)) {
      console.log(chalk.green('✅ Setup updated successfully!'));
    } else {
      console.log(chalk.red('❌ Failed to update setup'));
    }
  }

  async deleteSetup(setupId) {
    const setupDetails = this.dataManager.getSetupWithDetails(setupId);
    const { confirm } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'confirm',
        message: `Are you sure you want to delete ${setupDetails.setup.getDisplayName()}?`,
        default: false
      }
    ]);

    if (confirm) {
      const result = this.dataManager.deleteSetup(setupId);
      if (result.success) {
        console.log(chalk.green('✅ Setup deleted successfully!'));
      } else {
        console.log(chalk.red(`❌ ${result.message}`));
      }
    }
  }
}
