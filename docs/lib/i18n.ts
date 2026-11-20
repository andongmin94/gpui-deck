import { defineI18n } from 'fumadocs-core/i18n';

// Static export has no middleware, so every page carries its locale prefix.
export const i18n = defineI18n({
  defaultLanguage: 'en',
  languages: ['en', 'ko'],
  hideLocale: 'never',
  parser: 'dot',
});
