import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import { useAppStore } from './store';
import { api } from './api';
import './index.css';

// TODO: 临时调试用，验证完毕后删除
(window as unknown as Record<string, unknown>).__store = useAppStore;
(window as unknown as Record<string, unknown>).__api = api;

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
