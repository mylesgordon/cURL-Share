<script lang="ts">
	import { Visibility } from '$lib/types';
	import Meta from '$lib/components/Meta.svelte';
	import VisibilityInput from '$lib/components/VisibilityInput.svelte';
	import { Button, Input } from 'agnostic-svelte';
	let name: string;
	let description: string;
	let visibility: Visibility = Visibility.Private;

	function onSubmit(e: SubmitEvent) {
		console.log(typeof e);
	}
</script>

<Meta title="Create Project" />
<h2>Create a new project</h2>

<form class="flex flex-col space-y-3 pt-4" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="name" label="Name" bind:value={name} />
	<Input isRounded id="description" label="Description" type="textarea" bind:value={description} />
	<VisibilityInput bind:visibility={visibility} />

	<Button isBordered isRounded mode="primary" type="submit">Create</Button>
</form>
