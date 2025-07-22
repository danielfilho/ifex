import inquirer from 'inquirer';
import chalk from 'chalk';

export class CancelledError extends Error {
  constructor() {
    super('User cancelled');
    this.name = 'CancelledError';
  }
}

// Simple approach: enhance questions to allow "esc" as a valid input that triggers cancellation
export async function promptWithCancel(questions, message = 'Press Ctrl+C or type "esc" to cancel and go back') {
  console.log(chalk.gray(message));

  // Create enhanced questions that check for escape keywords
  const enhancedQuestions = questions.map(question => {
    if (question.type === 'input') {
      const originalValidate = question.validate || (() => true);
      return {
        ...question,
        validate: (input) => {
          // Check for escape keywords
          if (input && typeof input === 'string') {
            const trimmed = input.toLowerCase().trim();
            if (trimmed === 'esc' || trimmed === 'escape' || trimmed === 'back' || trimmed === 'cancel') {
              throw new CancelledError();
            }
          }
          return originalValidate(input);
        }
      };
    }
    return question;
  });

  try {
    const result = await inquirer.prompt(enhancedQuestions);
    return result;
  } catch (error) {
    if (error instanceof CancelledError) {
      throw error;
    }
    // Handle standard Ctrl+C cancellation
    if (error.isTtyError || error.name === 'ExitPromptError') {
      throw new CancelledError();
    }
    throw error;
  }
}

export function wrapWithCancelHandler(asyncFn) {
  return async function(...args) {
    try {
      return await asyncFn.apply(this, args);
    } catch (error) {
      if (error instanceof CancelledError) {
        console.log(chalk.yellow('\n⚠️  Operation cancelled'));
        return undefined;
      }
      throw error;
    }
  };
}
