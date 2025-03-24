import { createApp } from 'vue'
import App from './App.vue'

// 添加全局样式
const style = document.createElement('style');
style.textContent = `
  body {
    margin: 0;
    padding: 0;
    background: linear-gradient(135deg, #f5f7fa 0%, #e4e7eb 100%);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    min-height: 100vh;
    color: #2c3e50;
  }
  
  #app {
    width: 100vw;
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding: 20px 0;
    box-sizing: border-box;
  }

  ::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  ::-webkit-scrollbar-track {
    background: #f1f1f1;
    border-radius: 4px;
  }

  ::-webkit-scrollbar-thumb {
    background: #c0c4cc;
    border-radius: 4px;
  }

  ::-webkit-scrollbar-thumb:hover {
    background: #909399;
  }

  ::selection {
    background: rgba(52, 152, 219, 0.2);
  }
`;
document.head.appendChild(style);

createApp(App).mount('#app');
