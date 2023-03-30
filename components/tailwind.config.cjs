/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}', '../components/src/lib/**/*.svelte'],
	theme: {
		fontFamily: {
			primary: 'lato, san-serif',
			copy: 'lato, san-serif',
			sans: 'lato, san-serif',
			header: 'strokeWeight, sans-serif'
		},
		extend: {
			colors: {
				focus: '#1E6AFF',
				red: '#ff0000',
				green: '#00ff00',
				body: {
					DEFAULT: '#333333',
					light: '#586E75',
					dark: '#CCCCCC'
				},
				gray: {
					primary: '#CCCCCC',
					secondary: '#E6E6E6',
					'brand-1': '#586E75',
					'brand-2': '#657B83',
					'brand-3': '#839496',
					'brand-4': '#93A1A1',
					canvas: '#EEEEEE'
				},
				'design-brand-100': '#FEEDE1',
				'design-brand-200': '#FEDCC3',
				'design-brand-300': '#FDC7A0',
				'design-brand-400': '#FCAC73',
				'design-brand-500': '#FB8C3A',
				'design-brand-600': '#FA710F',
				'design-brand-700': '#E16005',
				'design-brand-800': '#B94F04',
				'design-brand-900': '#873A03',
				'stack-brand-100': '#E0F4FB',
				'stack-brand-200': '#C1EAF6',
				'stack-brand-300': '#98DCF0',
				'stack-brand-400': '#6CCCEA',
				'stack-brand-500': '#2FB6E1',
				'stack-brand-600': '#1EA4CD',
				'stack-brand-700': '#1A92B7',
				'stack-brand-800': '#167998',
				'stack-brand-900': '#0F566B',
				'brand-primary': '#2FB6E1',
				'brand-secondary': '#FB8C3A',
				'dark-1': '#002B36',
				'dark-2': '#073642',
				'secondary-red': '#E15B2F',
				'border-light': '#E6E6E6',
				'border-dark': '#586E75',
				node: '#333333',
				gradient: '#93A1A1',
				command: {
					DEFAULT: '#6CCCEA',
					light: '#C1EAF6',
					dark: '#2FB6E1'
				},
				event: {
					DEFAULT: '#FAAC74',
					light: '#FEDCC3',
					dark: '#FB8C3A'
				},
				readModel: {
					DEFAULT: '#79D04E',
					light: '#C5EBB2',
					dark: '#5FB831'
				},
				interfaceColor: {
					DEFAULT: '#F2F2F2',
					dark: '#E4E4E4'
				}
			},
			transitionProperty: {
				width: 'width',
				spacing: 'margin, padding, margin-left, margin-right'
			},
			minWidth: {
				placement: '98px',
				placementPadded: '142px'
			},
			minHeight: {
				placement: '98px',
				placementPadded: '142px'
			},
			fontSize: {
				xs: '0.75rem',
				default: '0.75rem',
				node: '0.8125rem'
			},
			boxShadow: {
				placement: '0 4px 6px -4px rgba(0,0,0,0.83)',
				interface: '0 2px 2px 0px rgba(0,0,0,0.19)',
				header: '0 2px 2px 0 rgba(0, 0, 0, 0.16)',
				'3xl': '0 35px 35px rgba(0, 0, 0, 0.25)',
				'4xl': [
					'0 2px 4px -1px rgba(0, 0, 0, 0.20)',
					'0 4px 5px 0 rgba(0, 0, 0, 0.14)',
					'0 1px 10px 0 rgba(0, 0, 0, 0.12)'
				]
			},
			container: {
				padding: {
					DEFAULT: '1rem',
					sm: '2rem',
					lg: '0rem'
				},
				center: true,
				screens: {
					sm: '640px',
					md: '768px',
					lg: '1024px',
					xl: '1280px',
					'2xl': '1280px'
				}
			},
			typography: (theme) => ({
				DEFAULT: {
					css: {
						fontFamily: theme('fontFamily.primary'),
						color: theme('colors.body.DEFAULT'),
						fontSize: theme('fontSize.default'),
						lineHeight: theme('lineHeight.normal'),
						fontWeight: theme('fontWeight.medium')
					}
				},
				invert: {
					css: {
						color: theme('colors.white')
					}
				}
			})
		}
	},
	plugins: [
		require('@tailwindcss/typography'),
		require('daisyui'),
		function ({ addComponents }) {
			addComponents({
				'.container': {
					maxWidth: '100%',
					'@screen sm': {
						maxWidth: '640px'
					},
					'@screen md': {
						maxWidth: '768px'
					},
					'@screen lg': {
						maxWidth: '1024px'
					},
					'@screen xl': {
						maxWidth: '1280px'
					},
					'@screen 2xl': {
						//makes the maximum width of the container 1280px without breaking the 1536px breakpoint
						maxWidth: '1280px'
					}
				}
			});
		}
	]
};
