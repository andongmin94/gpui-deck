import { defineI18nUI } from 'fumadocs-ui/i18n';
import { i18n } from './i18n';

export const i18nUI = defineI18nUI(i18n, {
  en: { displayName: 'English' },
  ko: {
    displayName: '한국어',
    'Search(search trigger)': '검색',
    'Search(search dialog)': '문서 검색',
    'No results found(search dialog)': '결과가 없습니다',
    'On this page(table of contents)': '이 페이지의 목차',
    'No Headings(table of contents)': '제목이 없습니다',
    'Next Page(pagination)': '다음',
    'Previous Page(pagination)': '이전',
    'Edit on GitHub(edit page)': 'GitHub에서 편집',
    'Choose a language(language switcher)': '언어 선택',
    'Language(language switcher)': '언어',
    'Theme(site menu)': '테마',
    'Light(theme switcher)(aria-label)': '라이트',
    'Dark(theme switcher)(aria-label)': '다크',
    'System(theme switcher)(aria-label)': '시스템',
    'Toggle Theme(theme switcher)(aria-label)': '테마 전환',
    'Page Not Found(404 not found page)': '페이지를 찾을 수 없습니다',
    'Back to Home(404 not found page)': '홈으로',
    'Copy Text(code block)(aria-label)': '코드 복사',
    'Copied Text(code block)(aria-label)': '복사됨',
    'Skip to Content(skip link)': '본문으로 건너뛰기',
    'Table of Contents(inline table of contents)': '목차',
  },
});
