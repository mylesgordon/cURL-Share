import { get } from 'svelte/store';
import { internalBackendUrl } from './stores';
import type { CurlGroup, Fetch, Project, ProjectInfo } from './types';

const url = get(internalBackendUrl);

type ProjectAdminStatus = {
	isUserAdmin: boolean;
};

export async function fetchCurlGroup(fetch: Fetch, curlGroupId: string): Promise<CurlGroup> {
	const response = await fetch(`${url}/api/v1/group/${curlGroupId}`, {
		method: 'GET',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});

	return await response.json();
}

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

export async function fetchProjectAdminStatus(
	fetch: Fetch,
	projectId: string
): Promise<ProjectAdminStatus> {
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
