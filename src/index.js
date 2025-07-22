#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';
import { readFileSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import { CLI } from './cli.js';
import { ExifManager } from './exif.js';
import exifr from 'exifr';

const __dirname = dirname(fileURLToPath(import.meta.url));
const packageJson = JSON.parse(readFileSync(join(__dirname, '../package.json'), 'utf8'));

const program = new Command();
const cli = new CLI();
const exifManager = new ExifManager();

program
  .name('ifex')
  .description('A modern CLI tool for managing EXIF data in JPEG files')
  .version(packageJson.version);

program
  .command('run')
  .description('Interactive mode to apply or erase EXIF data')
  .action(runInteractive);

program
  .command('manage')
  .description('Manage cameras, lenses, films, and setups')
  .action(async() => {
    try {
      console.log(chalk.blue('🏷️  IFEX - Equipment Manager\n'));
      while (true) {
        const result = await cli.managementCLI.showMainManagementMenu();
        if (result === 'back') {
          break;
        }
      }
    } catch (error) {
      console.error(chalk.red('An error occurred:'), error.message);
      process.exit(1);
    }
  });

program
  .command('check')
  .description('Display EXIF data from an image file in a formatted table')
  .argument('<file>', 'Path to the image file')
  .action(async(filePath) => {
    try {
      await showExifData(filePath);
    } catch (error) {
      console.error(chalk.red('Error reading EXIF data:'), error.message);
      process.exit(1);
    }
  });

async function showExifData(filePath) {
  console.log(chalk.blue(`📷 EXIF Data for: ${chalk.white(filePath)}\n`));

  try {
    // Read all EXIF data
    const exifData = await exifr.parse(filePath);

    if (!exifData || Object.keys(exifData).length === 0) {
      console.log(chalk.yellow('⚠️  No EXIF data found in this image.'));
      return;
    }

    // Convert EXIF data to table format
    const exifEntries = [];

    function processExifObject(obj, prefix = '') {
      for (const [key, value] of Object.entries(obj)) {
        const fullKey = prefix ? `${prefix}.${key}` : key;

        if (value && typeof value === 'object' && !Array.isArray(value) && !(value instanceof Date)) {
          // Nested object - recurse
          processExifObject(value, fullKey);
        } else {
          // Format the value
          let formattedValue = value;
          if (value instanceof Date) {
            formattedValue = value.toLocaleString();
          } else if (Array.isArray(value)) {
            formattedValue = value.join(', ');
          } else if (typeof value === 'number') {
            formattedValue = value.toString();
          } else if (typeof value === 'string') {
            formattedValue = value;
          } else {
            formattedValue = String(value);
          }

          exifEntries.push({
            tag: fullKey,
            value: formattedValue
          });
        }
      }
    }

    processExifObject(exifData);

    // Sort entries by tag name
    exifEntries.sort((a, b) => a.tag.localeCompare(b.tag));

    // Display table
    displayTable(exifEntries);

  } catch (error) {
    console.error(chalk.red('Error reading EXIF data:'), error.message);
    throw error;
  }
}

function displayTable(entries) {
  console.log(chalk.blue(`📷 EXIF Data (${entries.length} entries)\n`));

  // Calculate column widths
  const maxTagLength = Math.max(...entries.map(entry => entry.tag.length), 15);
  const maxValueLength = Math.max(...entries.map(entry => entry.value.toString().length), 20);

  // Table header
  const tagHeader = 'EXIF Tag'.padEnd(maxTagLength);
  const valueHeader = 'Value'.padEnd(maxValueLength);
  console.log(chalk.cyan(`┌─${'─'.repeat(maxTagLength)}─┬─${'─'.repeat(maxValueLength)}─┐`));
  console.log(chalk.cyan(`│ ${tagHeader} │ ${valueHeader} │`));
  console.log(chalk.cyan(`├─${'─'.repeat(maxTagLength)}─┼─${'─'.repeat(maxValueLength)}─┤`));

  // Table rows
  for (const entry of entries) {
    const tag = entry.tag.padEnd(maxTagLength);
    const value = entry.value.toString().padEnd(maxValueLength);
    console.log(`│ ${chalk.yellow(tag)} │ ${chalk.white(value)} │`);
  }

  // Table footer
  console.log(chalk.cyan(`└─${'─'.repeat(maxTagLength)}─┴─${'─'.repeat(maxValueLength)}─┘`));
}

async function runInteractive() {
  try {
    console.log(chalk.blue('🏷️  IFEX - EXIF Data Manager\n'));

    while (true) {
      const action = await cli.promptMainMenu();

      if (action === 'exit') {
        console.log(chalk.blue('👋 Goodbye!'));
        break;
      }

      if (action === 'manage') {
        while (true) {
          const result = await cli.managementCLI.showMainManagementMenu();
          if (result === 'back') {
            break;
          }
        }
        continue;
      }

      if (action === 'erase') {
        const folderPath = await cli.promptFolderPath();
        const recursive = await cli.promptRecursiveProcessing();
        const confirmed = await cli.confirmEraseExif();

        if (!confirmed) {
          console.log(chalk.yellow('Operation cancelled.'));
          continue;
        }

        console.log(chalk.blue('\n🗑️  Erasing EXIF data...\n'));
        const result = await exifManager.processFolder(folderPath, null, 'erase', recursive);

        if (result.success) {
          console.log(chalk.green(`✅ Successfully processed ${result.results.processed} files`));
          if (result.results.failed > 0) {
            console.log(chalk.red(`❌ Failed to process ${result.results.failed} files`));
          }
        } else {
          console.log(chalk.red(`❌ Error: ${result.message}`));
        }
        continue;
      }

      if (action === 'apply') {
        const selection = await cli.selectSetupAndFilm();
        if (!selection) {
          console.log(chalk.yellow('No valid setup and film selected. Returning to main menu.'));
          continue;
        }

        cli.displaySelection(selection);

        const folderPath = await cli.promptFolderPath();
        const recursive = await cli.promptRecursiveProcessing();

        console.log(chalk.blue('\n📝 Applying EXIF data...\n'));
        const result = await exifManager.processFolder(folderPath, selection, 'apply', recursive);

        if (result.success) {
          console.log(chalk.green(`✅ Successfully processed ${result.results.processed} files`));
          if (result.results.failed > 0) {
            console.log(chalk.red(`❌ Failed to process ${result.results.failed} files`));
          }

          console.log('\n📊 Processing Results:');
          result.results.files.forEach(file => {
            const status = file.success ? chalk.green('✓') : chalk.red('✗');
            const typeLabel = file.type ? `[${file.type.toUpperCase()}]` : '';
            console.log(`  ${status} ${file.name} ${chalk.gray(typeLabel)}`);
          });
        } else {
          console.log(chalk.red(`❌ Error: ${result.message}`));
        }
        continue;
      }
    }
  } catch (error) {
    console.error(chalk.red('An error occurred:'), error.message);
    process.exit(1);
  }
}

if (process.argv.length === 2) {
  runInteractive();
} else {
  program.parse();
}
