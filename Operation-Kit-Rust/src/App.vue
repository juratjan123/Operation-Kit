<script setup lang="ts">
import { ref, reactive, computed } from 'vue';
import type { AppState } from './types';
import { createTextState, updateTextState, goToPage } from './utils/text';
import * as api from './utils/api';
import { clipboard } from '@tauri-apps/api';
import {
  NButton,
  NInput,
  // NSpace,
  // NGrid,
  // NGridItem,
  NConfigProvider,
  darkTheme,
  lightTheme,
  useOsTheme,
  createDiscreteApi,
} from 'naive-ui';

const osTheme = useOsTheme();
const isDark = ref(osTheme.value === 'dark');

// 防抖控制
const messageDebounce = {
  timer: {} as Record<string, number>,
  lastMessage: {} as Record<string, string>,
  show(type: 'success' | 'error', key: string, msg: string) {
    // 如果当前有相同的消息正在显示，则不重复显示
    if (this.timer[key] && this.lastMessage[key] === msg) {
      return;
    }
    
    // 清除之前的定时器
    if (this.timer[key]) {
      clearTimeout(this.timer[key]);
    }
    
    // 显示消息
    message[type](msg);
    
    // 记录当前消息和定时器
    this.lastMessage[key] = msg;
    this.timer[key] = window.setTimeout(() => {
      delete this.lastMessage[key];
      delete this.timer[key];
    }, 3000); // 3秒后清除记录
  }
};

// 创建全局消息API
const { message } = createDiscreteApi(
  ['message'],
  {
    configProviderProps: computed(() => ({
      theme: isDark.value ? darkTheme : lightTheme,
    }))
  }
);

const state = reactive<AppState>({
  input: createTextState(),
  output: createTextState(),
});

// 输入框处理
async function handleInputChange(value: string) {
  updateTextState(state.input, value, true);
}

async function handlePaste() {
  try {
    const text = await clipboard.readText();
    if (text) {
      updateTextState(state.input, text);
      messageDebounce.show('success', 'paste', '粘贴成功');
    }
  } catch (err) {
    messageDebounce.show('error', 'paste', '读取剪贴板失败');
  }
}

// 页面导航
function goToInputPage(page: number) {
  goToPage(state.input, page);
}

function goToOutputPage(page: number) {
  goToPage(state.output, page);
}

// 文本处理功能
async function handleEncrypt() {
  try {
    const result = await api.encryptBatch(state.input.fullContent);
    updateTextState(state.output, result);
    messageDebounce.show('success', 'encrypt', '加密成功');
  } catch (err) {
    messageDebounce.show('error', 'encrypt', '加密失败');
  }
}

async function handleDecrypt() {
  try {
    const result = await api.decryptBatch(state.input.fullContent);
    updateTextState(state.output, result);
    messageDebounce.show('success', 'decrypt', '解密成功');
  } catch (err) {
    messageDebounce.show('error', 'decrypt', '解密失败');
  }
}

async function handleConvertFormat() {
  try {
    const result = await api.convertFormat(state.input.fullContent);
    updateTextState(state.output, result);
    messageDebounce.show('success', 'format', '格式转换成功');
  } catch (err) {
    messageDebounce.show('error', 'format', '格式转换失败');
  }
}

async function handleReplaceCommas() {
  try {
    const result = await api.replaceCommas(state.input.fullContent);
    if (result) {
      state.input.fullContent = result;
      state.input.currentContent = result;
      messageDebounce.show('success', 'replace', '替换完成');
    }
  } catch (err) {
    messageDebounce.show('error', 'replace', '替换逗号失败');
  }
}

async function handleAddQuotes(target: 'input' | 'output') {
  try {
    const text = target === 'input' ? state.input.fullContent : state.output.fullContent;
    const result = await api.addQuotes(text);
    updateTextState(target === 'input' ? state.input : state.output, result);
    messageDebounce.show('success', `addQuotes-${target}`, '添加引号成功');
  } catch (err) {
    messageDebounce.show('error', `addQuotes-${target}`, '添加引号失败');
  }
}

async function handleRemoveQuotes(target: 'input' | 'output') {
  try {
    const text = target === 'input' ? state.input.fullContent : state.output.fullContent;
    const result = await api.removeQuotes(text);
    updateTextState(target === 'input' ? state.input : state.output, result);
    messageDebounce.show('success', `removeQuotes-${target}`, '移除引号成功');
  } catch (err) {
    messageDebounce.show('error', `removeQuotes-${target}`, '移除引号失败');
  }
}

async function handleCopy() {
  try {
    await clipboard.writeText(state.output.fullContent);
    messageDebounce.show('success', 'copy', '已复制到剪贴板');
  } catch (err) {
    messageDebounce.show('error', 'copy', '复制失败');
  }
}

function clearText(target: 'input' | 'output') {
  updateTextState(target === 'input' ? state.input : state.output, '');
  messageDebounce.show('success', `clear-${target}`, `已清空${target === 'input' ? '输入' : '输出'}内容`);
}
</script>

<template>
  <n-config-provider :theme="isDark ? darkTheme : lightTheme">
    <div class="app-container" :class="{ 'dark': isDark }">
      <div class="content">
        <div class="section">
          <h2>输入内容</h2>
          <n-input
            v-model:value="state.input.currentContent"
            type="textarea"
            placeholder="请输入内容（支持换行或逗号分隔，如果数据量过大，请使用一键粘贴功能）"
            :autosize="false"
            class="text-area"
            @update:value="handleInputChange"
          />
          
          <div class="button-group" v-if="state.input.pageInfo.totalPages > 1">
            <n-button class="nav-button" @click="goToInputPage(1)">首页</n-button>
            <n-button class="nav-button" @click="goToInputPage(state.input.pageInfo.currentPage - 1)">上一页</n-button>
            <div class="page-info">{{ state.input.pageInfo.currentPage }}/{{ state.input.pageInfo.totalPages }}</div>
            <n-button class="nav-button" @click="goToInputPage(state.input.pageInfo.currentPage + 1)">下一页</n-button>
            <n-button class="nav-button" @click="goToInputPage(state.input.pageInfo.totalPages)">末页</n-button>
          </div>

          <div class="button-group">
            <n-button class="action-button" @click="handleReplaceCommas">中文逗号替换为英文</n-button>
            <n-button class="action-button" @click="() => handleAddQuotes('input')">添加单引号</n-button>
            <n-button class="action-button" @click="() => handleRemoveQuotes('input')">去除引号</n-button>
            <n-button class="action-button" @click="handlePaste">一键粘贴</n-button>
          </div>
        </div>

        <div class="button-group center">
          <n-button class="main-button" type="primary" @click="handleEncrypt">加密</n-button>
          <n-button class="main-button" type="primary" @click="handleDecrypt">解密</n-button>
          <n-button class="main-button" type="primary" @click="handleConvertFormat">转换格式</n-button>
        </div>

        <div class="section">
          <h2>输出内容</h2>
          <n-input
            v-model:value="state.output.currentContent"
            type="textarea"
            placeholder="输出结果将在这里显示"
            :autosize="false"
            class="text-area"
            readonly
          />

          <div class="button-group" v-if="state.output.pageInfo.totalPages > 1">
            <n-button class="nav-button" @click="goToOutputPage(1)">首页</n-button>
            <n-button class="nav-button" @click="goToOutputPage(state.output.pageInfo.currentPage - 1)">上一页</n-button>
            <div class="page-info">{{ state.output.pageInfo.currentPage }}/{{ state.output.pageInfo.totalPages }}</div>
            <n-button class="nav-button" @click="goToOutputPage(state.output.pageInfo.currentPage + 1)">下一页</n-button>
            <n-button class="nav-button" @click="goToOutputPage(state.output.pageInfo.totalPages)">末页</n-button>
          </div>

          <div class="button-group">
            <n-button class="action-button" @click="() => handleAddQuotes('output')">添加单引号</n-button>
            <n-button class="action-button" @click="() => handleRemoveQuotes('output')">去除引号</n-button>
            <n-button class="action-button" @click="handleCopy">一键复制</n-button>
          </div>
        </div>

        <div class="button-group center">
          <n-button class="clear-button" @click="() => clearText('input')">清空输入</n-button>
          <n-button class="clear-button" @click="() => clearText('output')">清空输出</n-button>
        </div>
      </div>
    </div>
  </n-config-provider>
</template>

<style>
:root {
  --app-bg-light: #f5f5f5;
  --app-bg-dark: #1a1a1a;
  --content-bg-light: #ffffff;
  --content-bg-dark: #2c2c2c;
  --text-color-light: #333;
  --text-color-dark: #fff;
  --border-color-light: #e5e5e5;
  --border-color-dark: #3a3a3a;
  --button-bg-light: #f0f0f0;
  --button-bg-dark: #363636;
  --button-hover-bg-light: #e0e0e0;
  --button-hover-bg-dark: #404040;
  --primary-color: #0052d9;
  --primary-hover-color: #2b6ff2;
}

html, body {
  margin: 0 !important;
  padding: 0 !important;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

#app {
  padding: 0 !important;
  margin: 0 !important;
}

.n-config-provider {
  margin: 0 !important;
  padding: 0 !important;
  height: 100%;
  width: 100%;
}

.app-container {
  margin: 0 !important;
  padding: 0 !important;
  width: 100vw;
  height: 100vh;
  background-color: var(--app-bg-light);
  color: var(--text-color-light);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.app-container.dark {
  background-color: var(--app-bg-dark);
  color: var(--text-color-dark);
}

.content {
  width: 90%;
  height: 90%;
  max-width: 800px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 20px;
  box-sizing: border-box;
  background-color: var(--content-bg-light);
  border-radius: 8px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
}

.dark .content {
  background-color: var(--content-bg-dark);
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.2);
}

.section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
  min-height: 0;
}

h2 {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
  color: inherit;
}

.text-area {
  flex: 1;
  min-height: 0;
}

:deep(.n-input) {
  height: 100%;
}

:deep(.n-input-wrapper) {
  height: 100%;
}

:deep(.n-input__textarea-el) {
  height: 100% !important;
  resize: none !important;
}

.button-group {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.button-group.center {
  justify-content: center;
  padding: 12px 0;
}

.action-button {
  flex: 1;
  min-width: 120px;
}

.nav-button {
  flex: 1;
}

.main-button {
  min-width: 120px;
}

.clear-button {
  min-width: 100px;
}

:deep(.n-button) {
  border-radius: 4px;
  height: 32px;
  padding: 0 16px;
  background-color: var(--button-bg-light);
  border: 1px solid var(--border-color-light);
  transition: all 0.2s ease;
}

.dark :deep(.n-button) {
  background-color: var(--button-bg-dark);
  border-color: var(--border-color-dark);
  color: var(--text-color-dark);
}

:deep(.n-button:hover) {
  background-color: var(--button-hover-bg-light);
}

.dark :deep(.n-button:hover) {
  background-color: var(--button-hover-bg-dark);
}

:deep(.n-button.n-button--primary-type) {
  background-color: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
}

:deep(.n-button.n-button--primary-type:hover) {
  background-color: var(--primary-hover-color);
  border-color: var(--primary-hover-color);
}

:deep(.n-input) {
  background-color: transparent;
}

:deep(.n-input-wrapper) {
  border-radius: 4px;
  border-color: var(--border-color-light);
  background-color: var(--content-bg-light);
}

.dark :deep(.n-input-wrapper) {
  border-color: var(--border-color-dark);
  background-color: var(--content-bg-dark);
}

:deep(.n-input textarea) {
  color: var(--text-color-light);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  font-size: 14px;
  line-height: 1.6;
  padding: 8px 12px;
}

.dark :deep(.n-input textarea) {
  color: var(--text-color-dark);
}

:deep(.n-input textarea::placeholder) {
  color: #999;
}

.dark :deep(.n-input textarea::placeholder) {
  color: #666;
}

.page-info {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 12px;
  font-size: 14px;
  color: var(--text-color-light);
  background-color: var(--button-bg-light);
  border: 1px solid var(--border-color-light);
  border-radius: 4px;
  min-width: 60px;
}

.dark .page-info {
  color: var(--text-color-dark);
  background-color: var(--button-bg-dark);
  border-color: var(--border-color-dark);
}
</style>
