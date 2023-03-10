export type Fetch = typeof fetch;

export type CreateProjectResponse = {
	id: number;
};

export type CurlGroup = {
	id: number;
	curls: string;
	description: string;
	labels: string;
	name: string;
	project_id: number;
};

export type Project = {
	info: ProjectInfo;
	admins: Array<string>;
	collaborators: Array<string>;
	groups: Array<CurlGroup>;
};

export type ProjectInfo = {
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
