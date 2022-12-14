/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		fontFamily: {
			primary: 'lato, san-serif'
		},
		extend: {
			colors: {
				primary: '##1E6AFF',
				body: '#333333',
				'body-light': '#586E75',
				'gray-primary': '#CCCCCC',
				'gray-secondary': '#E6E6E6',
				'gray-brand-1': '#586E75',
				'gray-brand-2': '#657B83',
				'gray-brand-3': '#839496',
				'gray-brand-4': '#93A1A1',
				'gray-bg': '#EEEEEE',
				'onote-brand': '#2FB6E1'
			},
			fontSize: {
				xs: '0.75rem',
				default: '0.75rem'
			},
			boxShadow: {
				header: '0 2px 2px 0 rgba(0, 0, 0, 0.16)',
				'3xl': '0 35px 35px rgba(0, 0, 0, 0.25)',
				'4xl': [
					'0 2px 4px -1px rgba(0, 0, 0, 0.20)',
					'0 4px 5px 0 rgba(0, 0, 0, 0.14)',
					'0 1px 10px 0 rgba(0, 0, 0, 0.12)'
				]
			},
			typography: (theme) => ({
				DEFAULT: {
					css: {
						fontFamily: theme('fontFamily.primary'),
						color: theme('colors.body'),
						fontSize: theme('fontSize.default'),
						lineHeight: theme('lineHeight.normal'),
						fontWeight: theme('fontWeight.medium')
					}
				}
			})
		}
	},
	plugins: [require('@tailwindcss/typography'), require('daisyui')]
};
