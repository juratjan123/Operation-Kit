export interface PageState {
  currentPage: number;
  totalPages: number;
  pageSize: number;
}

export interface TextState {
  fullContent: string;
  currentContent: string;
  pageInfo: PageState;
}

export interface AppState {
  input: TextState;
  output: TextState;
} 