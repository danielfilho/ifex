import { test, describe } from 'node:test';
import assert from 'node:assert';
import { createFuzzySearchSource, createFuzzySearchPrompt, createFuzzySearchPromptWithInstructions } from '../src/searchUtils.js';

describe('Search Utils', () => {
  const sampleChoices = [
    { name: 'Canon AE-1 Program', value: 'canon_ae1p' },
    { name: 'Nikon FM2', value: 'nikon_fm2' },
    { name: 'Pentax K1000', value: 'pentax_k1000' },
    { name: 'Leica M6', value: 'leica_m6' }
  ];

  describe('createFuzzySearchSource', () => {
    test('should return all choices when input is empty', async() => {
      const source = createFuzzySearchSource(sampleChoices);
      const result = await source({}, '');

      assert.strictEqual(result.length, 4);
      assert.deepStrictEqual(result, sampleChoices);
    });

    test('should filter choices based on fuzzy search', async() => {
      const source = createFuzzySearchSource(sampleChoices);
      const result = await source({}, 'canon');

      assert.strictEqual(result.length, 1);
      assert.strictEqual(result[0].name, 'Canon AE-1 Program');
    });

    test('should handle partial matches', async() => {
      const source = createFuzzySearchSource(sampleChoices);
      const result = await source({}, 'program');

      assert.strictEqual(result.length, 1);
      assert.strictEqual(result[0].name, 'Canon AE-1 Program');
    });

    test('should handle case insensitive search', async() => {
      const source = createFuzzySearchSource(sampleChoices);
      const result = await source({}, 'LEICA');

      assert.strictEqual(result.length, 1);
      assert.strictEqual(result[0].name, 'Leica M6');
    });

    test('should return empty array for no matches', async() => {
      const source = createFuzzySearchSource(sampleChoices);
      const result = await source({}, 'xyz123');

      assert.strictEqual(result.length, 0);
    });
  });

  describe('createFuzzySearchPrompt', () => {
    test('should create autocomplete prompt configuration', () => {
      const prompt = createFuzzySearchPrompt('camera', 'Select camera', sampleChoices);

      assert.strictEqual(prompt.type, 'autocomplete');
      assert.strictEqual(prompt.name, 'camera');
      assert.strictEqual(prompt.message, 'Select camera');
      assert.strictEqual(typeof prompt.source, 'function');
      assert.strictEqual(prompt.pageSize, 10);
    });

    test('should accept custom options', () => {
      const prompt = createFuzzySearchPrompt('camera', 'Select camera', sampleChoices, {
        pageSize: 5,
        searchText: 'Custom searching...',
        emptyText: 'Custom no results'
      });

      assert.strictEqual(prompt.pageSize, 5);
      assert.strictEqual(prompt.searchText, 'Custom searching...');
      assert.strictEqual(prompt.emptyText, 'Custom no results');
    });
  });

  describe('createFuzzySearchPromptWithInstructions', () => {
    test('should add instruction text to message', () => {
      const prompt = createFuzzySearchPromptWithInstructions('camera', 'Select camera', sampleChoices);

      assert.strictEqual(prompt.message, 'Select camera (start typing to filter)');
      assert.strictEqual(prompt.type, 'autocomplete');
      assert.strictEqual(prompt.searchText, 'Filtering...');
      assert.strictEqual(prompt.emptyText, 'No matching items found');
    });
  });
});
