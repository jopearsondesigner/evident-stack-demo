export type ButtonType = 'button';

export interface NavbarType {
	name: string;
	href: string;
	rel?: string;
	child?: NavbarType[];
}
