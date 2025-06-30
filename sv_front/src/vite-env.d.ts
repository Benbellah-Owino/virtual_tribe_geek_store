/// <reference types="vite/client" />

declare module '*.mjs?raw' {
	const src: string;
	export default src;
}
