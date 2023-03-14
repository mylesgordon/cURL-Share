import { externalBackendUrl } from './stores';
import { get } from 'svelte/store';
import type { CreateProjectResponse, CurlGroup, Fetch, Project, ProjectInfo } from './types';

const url = get(externalBackendUrl);

export async function createCurlGroupRequest(
	fetch: Fetch,
	curlGroup: CurlGroup
): Promise<CreateProjectResponse> {
	const response = await fetch(`${url}/api/v1/project/${curlGroup.project_id}/group`, {
		method: 'POST',
		body: JSON.stringify(curlGroup),
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url, 'Content-Type': 'application/json' },
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
