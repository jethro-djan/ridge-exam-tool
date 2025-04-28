/** @type {import('tailwindcss'}.Config} */
module.exports = {
	content: {
		files: ["*.html", "./src/**/*.rs"],
		transform: { 
			rs: (content) => content.replace(/(?:^|\s)class:/g, ' '), 
		},
	},
	theme: {
		extend: {
			animation: {
				'fade-in': 'fadeIn 0.5s ease-out',
			},
			keyframes: {
				fadeIn: {
					'0%': { opacity: '0' },
					'100%': { opacity: '1' },
				}
			}
		}
	},
	plugins: [],
}
