import { attachConsole, warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

// Forward Rust logs (info!, warn!, error!, debug!) to browser devtools console.
// The Rust side is configured with TargetKind::Webview to send logs here.
attachConsole();

// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;
