import type { TextState } from '../types';

export const PAGE_SIZE = 5000;

export function calculatePages(text: string): number {
  return Math.ceil(text.length / PAGE_SIZE);
}

export function getPageContent(text: string, page: number): string {
  const start = (page - 1) * PAGE_SIZE;
  const end = page * PAGE_SIZE;
  return text.slice(start, end);
}

export function createTextState(): TextState {
  return {
    fullContent: '',
    currentContent: '',
    pageInfo: {
      currentPage: 1,
      totalPages: 1,
      pageSize: PAGE_SIZE
    }
  };
}

export function updateTextState(state: TextState, newContent: string, isFullUpdate = true) {
  if (isFullUpdate) {
    state.fullContent = newContent;
    state.pageInfo.totalPages = Math.max(1, calculatePages(newContent));
    state.pageInfo.currentPage = 1;
  }
  state.currentContent = getPageContent(state.fullContent, state.pageInfo.currentPage);
}

export function goToPage(state: TextState, page: number) {
  if (page < 1 || page > state.pageInfo.totalPages) return;
  state.pageInfo.currentPage = page;
  state.currentContent = getPageContent(state.fullContent, page);
} 