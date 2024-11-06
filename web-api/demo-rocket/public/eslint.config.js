const { ESLint } = require('eslint');

module.exports = new ESLint({
  ignorePatterns: [
    'assets/**',
    'css/**',
    'node_modules/**',
  ],
  baseConfig: {
    plugins: ['tailwindcss', 'html'],
    extends: ['plugin:tailwindcss/recommended'],
    rules: {
      'tailwindcss/classnames-order': 'warn',
      'tailwindcss/no-custom-classname': 'off',
    },
    env: {
      browser: true,
      es2021: true,
    },
    parserOptions: {
      ecmaVersion: 12,
      sourceType: 'module',
    },
    overrides: [
      {
        files: ['*.js', '*.ts', '*.html'],
        extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended'],
        plugins: ['@typescript-eslint'],
        rules: {
          'react/react-in-jsx-scope': 'off',
          '@typescript-eslint/explicit-module-boundary-types': 'off',
          'react/prop-types': 'off',
        },
        settings: {
          react: {
            version: 'detect',
          },
        },
      },
      {
        files: ['*.html'],
        extends: ['plugin:html/recommended'],
      }
    ],
  },
});
