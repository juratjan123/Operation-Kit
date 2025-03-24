import { invoke } from '@tauri-apps/api/tauri';

export async function encryptBatch(text: string): Promise<string> {
  return invoke<string>('process_batch_encrypt', { input: text });
}

export async function decryptBatch(text: string): Promise<string> {
  return invoke<string>('process_batch_decrypt', { input: text });
}

export async function convertFormat(text: string): Promise<string> {
  return invoke<string>('process_convert_format', { input: text });
}

export async function replaceCommas(text: string): Promise<string> {
  return invoke<string>('process_replace_commas', { input: text });
}

export async function addQuotes(text: string): Promise<string> {
  return invoke<string>('process_add_quotes', { input: text });
}

export async function removeQuotes(text: string): Promise<string> {
  return invoke<string>('process_remove_quotes', { input: text });
} 