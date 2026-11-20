import { source } from '@/lib/source';
import { createFromSource } from 'fumadocs-core/search/server';

export const revalidate = false;

// Orama has no Korean stemmer; the English tokenizer still splits on spaces.
export const { staticGET: GET } = createFromSource(source, {
  localeMap: {
    en: { language: 'english' },
    ko: { language: 'english' },
  },
});
