import Button from '../lib/Button.svelte';

export default {
	title: 'Global/Button',
	component: Button,
	argTypes: {
		backgroundColor: { control: 'color' },
		label: { control: 'text' },
		onClick: { action: 'onClick' },
		color: {
			control: { type: 'select' },
			options: ['blue', 'primary', 'secondary', 'disabled', 'onote-primary', 'onote-secondary']
		},
		size: {
			control: { type: 'select' },
			options: ['sm', 'md', 'lg']
		}
	}
};

const Template = (/** @type {{ onClick: any; }} */ args) => ({
	Component: Button,
	props: args,
	on: {
		click: args.onClick
	}
});

export const Primary = Template.bind({});
// @ts-ignore
Primary.args = {
	gradient: true,
	size: 'md',
	color: 'primary',
	label: 'Primary'
};

export const Secondary = Template.bind({});
// @ts-ignore
Secondary.args = {
	gradient: false,
	size: 'md',
	color: 'secondary',
	label: 'Secondary'
};

export const Large = Template.bind({});
// @ts-ignore
Large.args = {
	size: 'lg',
	color: 'default',
	label: 'Default'
};

export const Small = Template.bind({});
// @ts-ignore
Small.args = {
	size: 'sm',
	color: 'default',
	label: 'Default'
};

export const Warning = Template.bind({});
// @ts-ignore
Warning.args = {
	gradient: false,
	size: 'md',
	color: 'warning',
	label: 'Warning'
};

export const Success = Template.bind({});
// @ts-ignore
Success.args = {
	gradient: false,
	size: 'md',
	color: 'success',
	label: 'Success'
};

export const brandDesignPrimary = Template.bind({});
// @ts-ignore
brandDesignPrimary.args = {
	gradient: true,
	size: 'md',
	color: 'brandDesignPrimary',
	label: 'Brand Primary'
};

export const BrandSecondary = Template.bind({});
// @ts-ignore
BrandSecondary.args = {
	gradient: true,
	size: 'md',
	color: 'brandStackPrimary',
	label: 'Brand Secondary'
};

export const BrandSecondarySmall = Template.bind({});
// @ts-ignore
BrandSecondarySmall.args = {
	gradient: true,
	size: 'sm',
	color: 'brandStackPrimary',
	label: 'Sign Up'
};
