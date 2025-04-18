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

// 获取当前加密配置
export async function getCryptoConfig(): Promise<string> {
  return invoke<string>('get_crypto_config');
}

// 设置加密配置
export async function setCryptoConfig(configName: string): Promise<void> {
  return invoke<void>('set_crypto_config', { configName });
}

// 获取华为前缀配置
export async function getHuaweiPrefixConfig(): Promise<boolean> {
  return invoke<boolean>('get_huawei_prefix_config');
}

// 设置华为前缀配置
export async function setHuaweiPrefixConfig(usePrefix: boolean): Promise<void> {
  return invoke<void>('set_huawei_prefix_config', { usePrefix });
}

// 上传ID列表到OSS
export async function uploadToOSS(accessId: string, accessKey: string, content: string, channel: string): Promise<string> {
  return invoke<string>('upload_to_oss', { accessId, accessKey, content, channel });
} 