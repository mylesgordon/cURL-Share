import { backendUrl } from "./stores";
import { get } from "svelte/store";
import type { Project, ProjectInfo } from "./types";

const url = get(backendUrl);

type CreateProjectResponse = {
	id: number
}

export async function fetchIsLoggedIn(fetch: any): Promise<boolean> {
	const response = await fetch(`${url}/api/v1/user-status`, {
		method: 'GET',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});
	const responseJson = await response.json();
	return responseJson.is_logged_in ? responseJson.is_logged_in : false;
}

export async function fetchProject(fetch: any, projectId: string): Promise<Project> {
	const projectResponse = await fetch(`${url}/api/v1/project/${projectId}`, {
		method: 'GET',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});		

    return await projectResponse.json();
}

export async function fetchProjects(fetch: any): Promise<ProjectInfo[]> {
	const response = await fetch(`${url}/api/v1/project`, {
		method: 'GET',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});
	return await response.json();
}

export async function createProjectRequest(fetch: any, projectInfo: ProjectInfo): Promise<CreateProjectResponse> {
	const response = await fetch(`${url}/api/v1/project`, {
		method: 'POST',
		body: JSON.stringify(projectInfo),
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url, 'Content-Type': 'application/json' },
		credentials: 'include',
	});

    return await response.json();
}

export async function logInRequest(fetch: any, endpoint: string, username: string, password: string): Promise<number> {
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

export async function logOutRequest(fetch: any): Promise<void> {
	await fetch(`${url}/api/v1/log-out`, {
		method: 'POST',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': url },
		credentials: 'include'
	});
}