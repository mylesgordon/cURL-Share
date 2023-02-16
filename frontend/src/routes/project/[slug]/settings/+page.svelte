<script lang="ts">
	import { Button, Dialog, Input } from 'agnostic-svelte';
	import Meta from '$lib/components/Meta.svelte';
	import VisibilityInput from '$lib/components/VisibilityInput.svelte';

	import { splitAndTrim } from '$lib/common';
	import type { PageData } from './$types';
	import type { SvelteComponent } from 'svelte';
	export let data: PageData;

	let name = data.projectSettings.name;
	let description = data.projectSettings.description;
	let visibility = data.projectSettings.visibility;
	let collaborators = data.projectSettings.collaborators;
	let environments = data.projectSettings.environments;

	let collaboratorText = collaborators.join(',');
	let environmentText = environments.join(',');

	$: collaborators = splitAndTrim(collaboratorText);
	$: environments = splitAndTrim(environmentText);

	const headingAndTitle = `${name} settings`;

	function onSubmit(e: SubmitEvent) {
		console.log(typeof e);
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

	function deleteProject(e: SubmitEvent) {
		console.log(typeof e);
	}
</script>

<Meta title={headingAndTitle} />
<h1>{headingAndTitle}</h1>

<form class="flex flex-col space-y-3 pt-4" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="name" label="Name" bind:value={name} />
	<Input isRounded id="description" label="Description" bind:value={description} />
	<VisibilityInput bind:visibility />
	<Input isRounded id="collaborators" label="Collaborators" bind:value={collaboratorText} />
	<Input isRounded id="environments" label="Environments" bind:value={environmentText} />

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
		<h2>Are you sure you want to delete {data.projectSettings.name}?</h2>
		<form on:submit|preventDefault={deleteProject}>
			<Button isBordered isRounded mode="secondary" type="submit">Yes</Button>
			<Button isBordered isRounded mode="primary" on:click={closeDialog}>No</Button>
		</form>
	</div>
</Dialog>

{#if data.isUnitTest}
	<div id="dialog-root" />
{/if}
