import {defineConfig} from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
export default defineConfig({plugins:[react(),tailwindcss()],clearScreen:false,build:{rollupOptions:{output:{manualChunks:{charts:['recharts'],dialog:['@radix-ui/react-dialog']}}}},server:{port:1420,strictPort:true},envPrefix:['VITE_','TAURI_ENV_']});
