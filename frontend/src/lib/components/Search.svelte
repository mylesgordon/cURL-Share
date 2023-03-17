<script lang="ts">
	import { Input } from 'agnostic-svelte';
	import type { CurlGroup, ProjectInfo } from '$lib/types';

	export let curlGroupArray: Array<CurlGroup> = [];
	export let projectInfoArray: Array<ProjectInfo> = [];

	const initialCurlGroupArray = [...curlGroupArray];
	const initialProjectInfoArray = [...projectInfoArray];

	let inputValue = '';
	let selectedLabel = '';

	let labels: Array<string> = [
		...new Set(initialCurlGroupArray.map((group) => group.labels.split(',')).flat(1))
	];

	$: curlGroupArray = initialCurlGroupArray.filter((entry) => {
		const nameConstraint = entry.name.toLowerCase().includes(inputValue.toLowerCase());
		const labelConstraint = selectedLabel ? entry.labels.split(',').includes(selectedLabel) : true;
		return nameConstraint && labelConstraint;
	});
	$: projectInfoArray = initialProjectInfoArray.filter((entry) =>
		entry.name.toLowerCase().includes(inputValue.toLowerCase())
	);
</script>

<div class="flex">
	<Input
		bind:value={inputValue}
		placeholder="Search..."
		isLabelHidden={true}
		data-testid="search-input"
	/>

	{#if initialCurlGroupArray.length}
		<select name="labels" id="labels" data-testid="labels-select" bind:value={selectedLabel}>
			<option value="">Choose a label</option>
			{#each labels as label}
				<option value={label}>{label}</option>
			{/each}
		</select>
	{/if}
</div>

<style>
	#labels {
		padding: 6px;
	}
</style>
