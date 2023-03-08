<script lang="ts">
	import { Button, Input } from 'agnostic-svelte';
	import { Visibility, type projectInfo } from '$lib/types';
	import Meta from '$lib/components/Meta.svelte';
	import VisibilityInput from '$lib/components/VisibilityInput.svelte';
	import { goto } from '$app/navigation';
	import { backendUrl } from '$lib/stores';
	let name: string;
	let description: string;
	let visibility: Visibility = Visibility.Public;

	async function sendCreateProjectRequest() {
		const projectToBeCreated: projectInfo = {
			id: 0,
			environments: "",
			description,
			name,
			visibility
		};

		const response = await fetch(`${$backendUrl}/api/v1/project`, {
			method: 'POST',
			body: JSON.stringify(projectToBeCreated),
			mode: 'cors',
			headers: { 'Access-Control-Allow-Origin': $backendUrl, 'Content-Type': 'application/json' },
			credentials: 'include',
		});

		return await response.json();
	}

	function onSubmit(e: SubmitEvent) {
		try {
			sendCreateProjectRequest().then((response) => {
				goto(`project/${response.id}`);
			})
		} catch (e) {
			console.error(e);
		}
	}
</script>

<Meta title="Create Project" />
<h2>Create a new project</h2>

<form class="flex flex-col space-y-3 pt-4" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="name" label="Name" bind:value={name} />
	<Input isRounded id="description" label="Description" type="textarea" bind:value={description} />
	<VisibilityInput bind:visibility />

	<Button isBordered isRounded mode="primary" type="submit">Create</Button>
</form>
