<script lang="ts">
	let testData = [
		{ name: 'Test123', raw_query: 'abc123', request_type: 'GET' },
		{ name: 'ASD', raw_query: 'abc123', request_type: 'GET' }
	];
	import { Button } from 'agnostic-svelte';
	import { flip } from 'svelte/animate';
	import { splitAndTrim } from '$lib/common';
	import Input from 'agnostic-svelte/components/Input/Input.svelte';

	let labels: Array<string> = [];
	$: labels = splitAndTrim(labelsText);

	let description: string;
	let labelsText: string = labels.join('/');
	let name: string;

	// cURLs
	let hovering = -1;

	$: console.log(testData);

	function onDragStart(event: DragEvent, index: number) {
		if (event.dataTransfer === null) {
			return;
		}
		event.dataTransfer.effectAllowed = 'move';
		event.dataTransfer.dropEffect = 'move';
		const start = index;
		event.dataTransfer.setData('text/plain', start.toString());
	}

	function onDrop(event: DragEvent, target: number) {
		if (event.dataTransfer === null) {
			return;
		}
		event.dataTransfer.dropEffect = 'move';
		const start = parseInt(event.dataTransfer.getData('text/plain'));
		const newTracklist = testData;

		if (start < target) {
			newTracklist.splice(target + 1, 0, newTracklist[start]);
			newTracklist.splice(start, 1);
		} else {
			newTracklist.splice(target, 0, newTracklist[start]);
			newTracklist.splice(start + 1, 1);
		}
		testData = newTracklist;
		hovering = -1;
	}

	function onKeyDown(event: KeyboardEvent, _index: number) {
		if (event.repeat) {
			return;
		}
		switch (event.key) {
			case 'ArrowUp':
				console.log('arrow up!');
				break;
			case 'ArrowDown':
				console.log('arrow down!');
				break;
			default:
				break;
		}
	}

	function onSubmit(_e: SubmitEvent) {
		console.log('submit');
	}
</script>

<form class="flex flex-col space-y-3 p-4" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="name" label="Name" bind:value={name} />
	<Input isRounded id="description" label="Description" bind:value={description} />
	<Input isRounded id="labels" label="Labels" bind:value={labelsText} />

	<span class="input">cURLs</span>
	<!-- TODO: aria-describedby -->

	<ol>
		{#each testData as data, index (data.name)}
			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			<li
				animate:flip
				class:is-active={hovering === index}
				draggable="true"
				on:drop|preventDefault={(event) => onDrop(event, index)}
				on:dragenter={() => (hovering = index)}
				on:dragover|preventDefault
				on:dragstart={(event) => onDragStart(event, index)}
				on:keydown={(event) => onKeyDown(event, index)}
				tabindex="0"
			>
				{data.name}
				{data.raw_query}
				{data.request_type}
			</li>
		{/each}
	</ol>

	<Button isBordered isRounded mode="primary" type="submit">Create</Button>
</form>

<style>
	/* taken from https://github.com/AgnosticUI/agnosticui/blob/master/agnostic-svelte-ts/src/lib/components/Input/Input.svelte */
	.input {
		color: var(--agnostic-font-color, var(--agnostic-dark));
		font-family: var(--agnostic-font-family-body);
		font-weight: var(--agnostic-font-weight, 300);
		font-size: var(--agnostic-font-size, 1rem);
		line-height: var(--agnostic-line-height, var(--fluid-20, 1.25rem));
	}

	.is-active {
		background-color: aquamarine;
	}
</style>
