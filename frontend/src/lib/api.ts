import { backendUrl } from './stores';
import { get } from 'svelte/store';
import type { Project, ProjectInfo } from './types';

const url = get(backendUrl);

type CreateProjectResponse = {
	id: number;
};

type ProjectAdminStatus = {
	isUserAdmin: boolean;
}

type Fetch = typeof fetch;

export async function fetchIsLoggedIn(fetch: Fetch): Promise<boolean> {
	const response = await fetch(`${url}/api/v1/user-status`, {
		method: 'GET',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});
	const responseJson = await response.json();
	return responseJson.is_logged_in ? responseJson.is_logged_in : false;
}

export async function fetchProject(fetch: Fetch, projectId: string): Promise<Project> {
	const projectResponse = await fetch(`${url}/api/v1/project/${projectId}`, {
		method: 'GET',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});

	return await projectResponse.json();
}

export async function fetchProjectAdminStatus(fetch: Fetch, projectId: string): Promise<ProjectAdminStatus> {
	const projectResponse = await fetch(`${url}/api/v1/project/${projectId}/is-user-admin`, {
		method: 'GET',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});

	return await projectResponse.json();
}

export async function fetchProjects(fetch: Fetch): Promise<ProjectInfo[]> {
	const response = await fetch(`${url}/api/v1/project`, {
		method: 'GET',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});
	return await response.json();
}

export async function createProjectRequest(
	fetch: Fetch,
	projectInfo: ProjectInfo
): Promise<CreateProjectResponse> {
	const response = await fetch(`${url}/api/v1/project`, {
		method: 'POST',
		body: JSON.stringify(projectInfo),
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url, 'Content-Type': 'application/json' },
		credentials: 'include'
	});

	return await response.json();
}

export async function deleteProjectRequest(fetch: Fetch, project_id: number): Promise<void> {
	await fetch(`${url}/api/v1/project/${project_id}`, {
		method: 'DELETE',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});
}

export async function updateProjectRequest(fetch: Fetch, project: Project): Promise<void> {
	await fetch(`${url}/api/v1/project/${project.info.id}`, {
		method: 'POST',
		body: JSON.stringify(project),
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url, 'Content-Type': 'application/json' },
		credentials: 'include'
	});
}

export async function logInRequest(
	fetch: Fetch,
	endpoint: string,
	username: string,
	password: string
): Promise<number> {
	const request = await fetch(`${url}/api/v1/${endpoint}`, {
		method: 'POST',
		mode: 'cors',
		headers: {
			'Access-Control-Allow-Origin': url,
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ username, password }),
		credentials: 'include'
	});

	return request.status;
}

export async function logOutRequest(fetch: Fetch): Promise<void> {
	await fetch(`${url}/api/v1/log-out`, {
		method: 'POST',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});
}
