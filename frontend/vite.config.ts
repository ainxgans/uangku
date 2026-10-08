import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';
import adapter from '@sveltejs/adapter-node';

export default defineConfig({
	plugins: [
		sveltekit({
			adapter: adapter(),
			paths: {
				base: process.env.BASE_PATH || '/uangku'
			}
		}), 
		tailwindcss()
	]
});