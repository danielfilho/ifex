import fs from 'fs';
import path from 'path';
import { isSupportedFile, getFileType } from './fileTypes.js';

/**
 * File collection and discovery utilities
 */
export class FileCollector {
  collectFilesRecursively(folderPath, recursive = false) {
    const files = [];

    const scanDirectory = (dirPath) => {
      try {
        const entries = fs.readdirSync(dirPath);

        for (const entry of entries) {
          const fullPath = path.join(dirPath, entry);
          const stat = fs.statSync(fullPath);

          if (stat.isFile() && isSupportedFile(entry)) {
            const relativePath = path.relative(folderPath, fullPath);
            files.push({
              name: relativePath,
              path: fullPath,
              directory: path.dirname(relativePath) || '.'
            });
          } else if (stat.isDirectory() && recursive) {
            scanDirectory(fullPath);
          }
        }
      } catch (error) {
        console.warn(`Warning: Could not read directory ${dirPath}: ${error.message}`);
      }
    };

    scanDirectory(folderPath);
    return files;
  }

  async processFolder(folderPath, selection, action, recursive = false) {
    try {
      const supportedFiles = this.collectFilesRecursively(folderPath, recursive);

      if (supportedFiles.length === 0) {
        const recursiveText = recursive ? ' (including subdirectories)' : '';
        return {
          success: false,
          message: `No supported image files (JPEG, TIFF, DNG, RAW) found in the specified folder${recursiveText}.`
        };
      }

      const recursiveText = recursive ? ' (including subdirectories)' : '';
      console.log(`Found ${supportedFiles.length} supported image files${recursiveText}:`);

      this.displayFileSummary(supportedFiles, recursive);

      const results = {
        total: supportedFiles.length,
        processed: 0,
        failed: 0,
        files: []
      };

      // Process each file with the appropriate processor
      const { ExifManager } = await import('./exifManager.js');
      const exifManager = new ExifManager();

      for (const file of supportedFiles) {
        let success;

        if (action === 'apply') {
          success = await exifManager.applyExifToFile(file.path, selection);
        } else if (action === 'erase') {
          success = await exifManager.eraseExifFromFile(file.path);
        }

        results.files.push({
          name: file.name,
          type: getFileType(file.name),
          directory: file.directory,
          success
        });

        if (success) {
          results.processed++;
        } else {
          results.failed++;
        }
      }

      return { success: true, results };
    } catch (error) {
      return { success: false, message: error.message };
    }
  }

  displayFileSummary(supportedFiles, recursive) {
    const fileTypeCount = supportedFiles.reduce((acc, file) => {
      const type = getFileType(file.name);
      acc[type] = (acc[type] || 0) + 1;
      return acc;
    }, {});

    Object.entries(fileTypeCount).forEach(([type, count]) => {
      console.log(`  ${type.toUpperCase()}: ${count} files`);
    });

    if (recursive) {
      const directoryCounts = supportedFiles.reduce((acc, file) => {
        const dir = file.directory === '.' ? 'root' : file.directory;
        acc[dir] = (acc[dir] || 0) + 1;
        return acc;
      }, {});

      console.log('\nDirectory breakdown:');
      Object.entries(directoryCounts).forEach(([dir, count]) => {
        console.log(`  ${dir}: ${count} files`);
      });
    }
  }
}
