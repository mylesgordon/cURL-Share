<script lang="ts">
	import { Visibility, VisibilityOptions } from '$lib/types';
	import Meta from '$lib/components/Meta.svelte';
	import { Button, ChoiceInput, Input } from 'agnostic-svelte';
	let name: string;
	let description: string;
	let visibility: Visibility = Visibility.Public;

	function onSubmit(e: SubmitEvent) {
		console.log(typeof e);
	}

	let visibilityCheckedOption: Array<string> = ['Public'];
	$: visibility = Visibility[visibilityCheckedOption[0] as Visibility];
</script>

<Meta title="Create Project" />
<h2>Create a new project</h2>

<form class="flex flex-col space-y-3 pt-4" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="name" label="Name" bind:value={name} />
	<Input isRounded id="description" label="Description" type="textarea" bind:value={description} />
	<ChoiceInput
		id="visibility"
		type="radio"
		options={VisibilityOptions}
		legendLabel="Visibility"
		checkedOptions={visibilityCheckedOption}
		bind:checked={visibilityCheckedOption}
	/>

	<Button isBordered isRounded mode="primary" type="submit">Create</Button>
</form>
