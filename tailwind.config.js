/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				'background-main': '#2D3250',
				secondary: '#424769',
				third: '#7077A1',
				accent: { 0: '#F9E8C9', 1: '#F6B17A' }
			}
		}
	},
	plugins: []
};
