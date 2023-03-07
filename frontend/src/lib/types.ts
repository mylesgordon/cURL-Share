export type projectInfo = {
	id: number;
	environments: string;
	description: string;
	name: string;
	visibility: string;
};

export enum Visibility {
	Public = 'Public',
	Private = 'Private'
}

export const VisibilityOptions = [
	{
		name: 'visibility',
		value: 'Public',
		label: 'Public'
	},
	{
		name: 'visibility',
		value: 'Private',
		label: 'Private'
	}
];
