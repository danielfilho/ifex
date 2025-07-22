import inquirer from 'inquirer';
import autocompletePrompt from 'inquirer-autocomplete-prompt';
import fuzzy from 'fuzzy';

// Register the autocomplete prompt type
inquirer.registerPrompt('autocomplete', autocompletePrompt);

/**
 * Creates a fuzzy search source function for inquirer autocomplete
 * @param {Array} choices - Array of choice objects with name and value properties
 * @param {string} searchProperty - Property to search on (default: 'name')
 * @returns {Function} Source function for inquirer autocomplete
 */
export function createFuzzySearchSource(choices, searchProperty = 'name') {
  return async function(answersSoFar, input) {
    input = input || '';

    // If no input, return all choices
    if (!input.trim()) {
      return choices;
    }

    // Perform fuzzy search
    const options = {
      extract: function(choice) {
        return choice[searchProperty] || choice.name || choice;
      }
    };

    const results = fuzzy.filter(input, choices, options);

    // Return the original choice objects, preserving their structure
    return results.map(result => result.original);
  };
}

/**
 * Creates an autocomplete prompt with fuzzy search
 * @param {string} name - The name of the prompt
 * @param {string} message - The message to display
 * @param {Array} choices - Array of choices
 * @param {Object} options - Additional options
 * @returns {Object} Inquirer prompt configuration
 */
export function createFuzzySearchPrompt(name, message, choices, options = {}) {
  return {
    type: 'autocomplete',
    name,
    message,
    source: createFuzzySearchSource(choices, options.searchProperty),
    pageSize: options.pageSize || 10,
    suggestOnly: options.suggestOnly || false,
    searchText: options.searchText || 'Searching...',
    emptyText: options.emptyText || 'No results found',
    validate: options.validate || undefined,
    when: options.when || undefined
  };
}

/**
 * Enhanced autocomplete prompt that shows instruction text
 * @param {string} name - The name of the prompt
 * @param {string} message - The message to display
 * @param {Array} choices - Array of choices
 * @param {Object} options - Additional options
 * @returns {Object} Inquirer prompt configuration
 */
export function createFuzzySearchPromptWithInstructions(name, message, choices, options = {}) {
  const instructionMessage = `${message} (start typing to filter)`;

  return createFuzzySearchPrompt(name, instructionMessage, choices, {
    ...options,
    searchText: options.searchText || 'Filtering...',
    emptyText: options.emptyText || 'No matching items found'
  });
}
