<script lang="ts">
	import { Button, Dialog, Input } from 'agnostic-svelte';
	import Meta from '$lib/components/Meta.svelte';
	import VisibilityInput from '$lib/components/VisibilityInput.svelte';

	import { deleteProjectRequest, updateProjectRequest } from '$lib/api';
	import { goto } from '$app/navigation';
	import { splitAndTrim } from '$lib/common';
	import type { PageData } from './$types';
	import type { SvelteComponent } from 'svelte';
	import type { Visibility } from '$lib/types';

	export let data: PageData;
	const { project, success, userAdminStatus } = data;

	let name = project.info.name;
	let description = project.info.description;
	let visibility = project.info.visibility as Visibility;
	let admins = project.admins;
	let collaborators = project.collaborators;
	let environments = project.info.environments;

	let adminText = admins?.join(',');
	let collaboratorText = collaborators?.join(',');

	$: admins = splitAndTrim(adminText);
	$: collaborators = splitAndTrim(collaboratorText);

	const headingAndTitle = `${name} settings`;

	async function onSubmit(_e: SubmitEvent) {
		try {
			await updateProjectRequest(window.fetch, {
				admins,
				collaborators,
				groups: project.groups,
				info: { id: project.info.id, environments, description, name, visibility }
			});
			goto(`/project/${project.info.id}`);
		} catch (e) {
			console.error(e);
		}
	}

	// Delete project dialogue
	let dialogInstance: SvelteComponent;
	function assignDialogInstance(event: CustomEvent<SvelteComponent>) {
		dialogInstance = event.detail.instance;
	}
	function closeDialog() {
		dialogInstance.hide();
	}
	function openDialog() {
		if (dialogInstance) {
			dialogInstance.show();
		}
	}

	async function deleteProject(_e: SubmitEvent) {
		try {
			await deleteProjectRequest(window.fetch, project.info.id);
			goto('/');
		} catch (e) {
			console.error(e);
		}
	}
</script>

{#if success && userAdminStatus.isUserAdmin}
	<Meta title={headingAndTitle} />
	<h1>{headingAndTitle}</h1>

	<form class="flex flex-col space-y-3 pt-4" on:submit|preventDefault={onSubmit}>
		<Input isRounded id="name" label="Name" bind:value={name} />
		<Input isRounded id="description" label="Description" bind:value={description} />
		<VisibilityInput bind:visibility />
		<Input isRounded id="admins" label="Administrators (comma separated)" bind:value={adminText} />
		<Input
			isRounded
			id="collaborators"
			label="Collaborators (comma separated)"
			bind:value={collaboratorText}
		/>
		<Input
			isRounded
			id="environments"
			label="Environments (comma separated)"
			bind:value={environments}
		/>

		<div class="flex flex-row gap-2 justify-center">
			<Button isBordered isRounded mode="primary" type="submit">Save</Button>
			<Button isBordered isRounded mode="secondary" on:click={openDialog}>Delete</Button>
		</div>
	</form>

	<Dialog
		id="a11y-dialog"
		title="Delete project"
		dialogRoot="#dialog-root"
		closeButtonLabel="Close delete dialog"
		closeButtonPosition="last"
		titleId="deleteProjectDialog"
		role="dialog"
		classNames={{ title: 'h4 mbe18 flex justify-center' }}
		isAnimationFadeIn
		on:instance={assignDialogInstance}
	>
		<div class="flex flex-col items-center gap-2">
			<h2>Are you sure you want to delete {project.info.name}?</h2>
			<form on:submit|preventDefault={deleteProject}>
				<Button isBordered isRounded mode="secondary" type="submit">Yes</Button>
				<Button isBordered isRounded mode="primary" on:click={closeDialog}>No</Button>
			</form>
		</div>
	</Dialog>
{:else}
	<p>Failed to load project settings</p>
{/if}

{#if data.isUnitTest}
	<div id="dialog-root" />
{/if}
