export default [
  {
    ignores: ['node_modules/', '*.json', '*.md', 'LICENSE', '.gitignore']
  },
  {
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: 'module',
      globals: {
        console: 'readonly',
        process: 'readonly',
        Buffer: 'readonly',
        __dirname: 'readonly',
        __filename: 'readonly',
        global: 'readonly',
        setTimeout: 'readonly',
        clearTimeout: 'readonly',
        setInterval: 'readonly',
        clearInterval: 'readonly'
      }
    },
    rules: {
      // Whitespace and formatting rules
      'no-trailing-spaces': 'error',
      'no-multiple-empty-lines': ['error', { max: 1, maxEOF: 1, maxBOF: 0 }],
      'eol-last': ['error', 'always'],
      'indent': ['error', 2, { SwitchCase: 1 }],
      'semi': ['error', 'always'],
      'quotes': ['error', 'single', { avoidEscape: true }],
      'comma-dangle': ['error', 'never'],
      'object-curly-spacing': ['error', 'always'],
      'array-bracket-spacing': ['error', 'never'],
      'space-before-blocks': 'error',
      'space-before-function-paren': ['error', 'never'],
      'space-in-parens': ['error', 'never'],
      'space-infix-ops': 'error',
      'keyword-spacing': 'error',
      
      // Code quality rules
      'no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
      'no-var': 'error',
      'prefer-const': 'error',
      'no-console': 'off', // Allow console.log in CLI app
      'no-undef': 'error',
      'no-unreachable': 'error',
      'no-duplicate-imports': 'error',
      'consistent-return': 'error',
      'eqeqeq': ['error', 'always'],
      'no-throw-literal': 'error',
      
      // ES6+ rules
      'arrow-spacing': 'error',
      'template-curly-spacing': 'error',
      'rest-spread-spacing': 'error'
    }
  },
  {
    files: ['test/**/*.js'],
    languageOptions: {
      globals: {
        describe: 'readonly',
        test: 'readonly',
        beforeEach: 'readonly',
        afterEach: 'readonly'
      }
    }
  }
];