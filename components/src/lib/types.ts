export type ButtonType = ('button'|'submit');

export interface NavbarType {
	name: string;
	href: string;
	rel?: string;
	child?: NavbarType[];
}

export declare const xs = 'xs';
export declare const sm = 'sm';
export declare const md = 'md';
export declare const lg = 'lg';
export declare const xl = 'xl';
export declare type SizeType = typeof xs | typeof sm | typeof md | typeof lg | typeof xl;

export type InputType =
	| 'color'
	| 'date'
	| 'datetime-local'
	| 'email'
	| 'file'
	| 'hidden'
	| 'image'
	| 'month'
	| 'number'
	| 'password'
	| 'reset'
	| 'submit'
	| 'tel'
	| 'text'
	| 'time'
	| 'url'
	| 'week'
	| 'search';

export declare type FormSizeType = typeof sm | typeof md | typeof lg;
