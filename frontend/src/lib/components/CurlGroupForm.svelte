<script lang="ts">
	import { Button } from 'agnostic-svelte';
	import { flip } from 'svelte/animate';
	import { splitAndTrim } from '$lib/common';
	import Input from 'agnostic-svelte/components/Input/Input.svelte';
	import Ellipsis from 'virtual:icons/logos/svelte-icon';

	enum ButtonType {
		Up = -1,
		Down = 1
	}
	type Test = {
		id: number;
		name: string;
		description: string;
		raw_query: string;
	};
	let testData: Array<Test> = [
		{
			id: 0,
			name: 'Test123',
			description: 'This query does something really cool - trust me.',
			raw_query: 'abc123'
		},
		{
			id: 1,
			name: 'ASD',
			description: 'This is something different. This query does something less cool.',
			raw_query: 'abc123'
		}
	];
	const flipDurationMs = 750;

	let labels: Array<string> = [];
	$: labels = splitAndTrim(labelsText);

	let description: string;
	let labelsText: string = labels.join('/');
	let name: string;
	let rawCurlInput: string;

	let maxId = Math.max(...testData.map((data) => data.id), 0);

	// cURLs
	let hovering = -1;

	$: console.log(testData);

	function addCurl() {
		maxId += 1;
		testData = [
			...testData,
			{
				id: maxId,
				name: `cURL #${maxId}`,
				description: 'Insert description here',
				raw_query: rawCurlInput
			}
		];
		rawCurlInput = '';
	}

	function onSubmit() {}

	function buttonClick(index: number, buttonType: ButtonType) {
		const itemDestination = (index + buttonType) as number;
		if (testData[itemDestination] === undefined) {
			return;
		}
		swapItems(index, itemDestination);
	}

	function swapItems(a: number, b: number) {
		[testData[a], testData[b]] = [testData[b], testData[a]];
		testData = testData;
	}

	function dragStart(event: DragEvent, itemIndex: number) {
		event.dataTransfer?.setData('text/plain', itemIndex.toString());
	}

	function drop(event: DragEvent, itemIndex: number) {
		const eventString = event.dataTransfer?.getData('text/plain') ?? '';
		const eventIndex = parseInt(eventString);

		if (eventIndex === undefined) {
			return;
		}

		swapItems(eventIndex, itemIndex);
	}
</script>

<form class="flex flex-col space-y-3 p-4" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="name" label="Name" bind:value={name} />
	<Input isRounded id="description" label="Description" bind:value={description} />
	<Input isRounded id="labels" label="Labels" bind:value={labelsText} />

	<span>cURLs</span>
	<!-- TODO: aria-describedby -->

	<ol class="flex flex-col space-y-2">
		{#each testData as data, index (data.id)}
			<li
				class="flex flex-col space-y-2 border rounded-md p-2"
				class:is-active={hovering === index}
				animate:flip={{ duration: flipDurationMs }}
			>
				<div
					aria-hidden="true"
					draggable="true"
					on:dragstart={(event) => dragStart(event, index)}
					on:dragenter|preventDefault
					on:dragover|preventDefault={() => {
						hovering = index;
					}}
					on:dragend={() => {
						hovering = -1;
					}}
					on:drop|preventDefault={(event) => {
						drop(event, index);
					}}
				>
					<UpDownLeftRight />
				</div>
			 
				<label for="name">Name:</label>
				<input id="name" class="input" bind:value={data.name} draggable="true" on:dragstart|preventDefault />

				<label for="description">Description:</label>
				<input id="description" class="input" bind:value={data.description} />

				<label for="raw-query">Raw cURL query:</label>
				<input id="raw-query" class="input" bind:value={data.raw_query} />
				
				<div class="flex justify-end space-x-2">
					<Button
						isRounded
						on:click={() => buttonClick(index, ButtonType.Up)}
						isDisabled={index === 0}
					>
						<span class="sr-only">Move {data.name} up</span>
						<svg width="16" height="16" focusable="false" aria-hidden="true">
							<use xlink:href="#icon--up" />
						</svg>
					</Button>
					<Button
						isRounded
						on:click={() => buttonClick(index, ButtonType.Down)}
						isDisabled={index === testData.length - 1}
					>
						<span class="sr-only">Move {data.name} down</span>
						<svg width="16" height="16" focusable="false" aria-hidden="true">
							<use xlink:href="#icon--down" />
						</svg>
					</Button>
				</div>
			</li>
		{/each}
	</ol>

	<div class="flex flex-row">
		<Input isRounded id="raw-curl" label="Add cURL query" bind:value={rawCurlInput} />
		<Button isRounded on:click={addCurl} css="ml-2 self-end add-button">Add</Button>
	</div>

	<Button isBordered isRounded mode="primary" type="submit">Create</Button>
</form>

<svg xmlns="http://www.w3.org/2000/svg" class="visuallyHidden" focusable="false" aria-hidden="true">
	<defs>
		<symbol id="icon--up" viewBox="0 0 14 14">
			<path fill="none" stroke="currentColor" stroke-width="2" d="M.7 10.1L7 3.8l6.3 6.3" />
		</symbol>
		<symbol id="icon--down" viewBox="0 0 14 14">
			<path fill="none" stroke="currentColor" stroke-width="2" d="M.7 3.8L7 10.1l6.3-6.3" />
		</symbol>
	</defs>
</svg>

<style>
	.add-button {
		height: 2.375rem;
	}

	/* There is an issue with undefined bindings with agnostic ui's svelte implementation.
	This is just a direct copy of the input, input-rounded and label css styles */
	.input {
		color: var(--agnostic-font-color, var(--agnostic-dark));
  		font-family: var(--agnostic-font-family-body);
  		font-weight: var(--agnostic-font-weight, 300);
  		font-size: var(--agnostic-font-size, 1rem);
  		line-height: var(--agnostic-line-height, var(--fluid-20, 1.25rem));
  		width: 100%;
  		max-width: 100%;
		border-style: solid;
  		border-width: var(--agnostic-input-border-size, 1px);
  		border-color: var(--agnostic-input-border-color, var(--agnostic-gray-light));
  		padding-block-start: var(--agnostic-input-vertical-pad, 0.5rem);
  		padding-block-end: var(--agnostic-input-vertical-pad, 0.5rem);
  		padding-inline-start: var(--agnostic-input-side-padding, 0.75rem);
  		padding-inline-end: var(--agnostic-input-side-padding, 0.75rem);
  		transition-property: box-shadow;
  		transition-duration: var(--agnostic-input-timing, var(--agnostic-timing-medium));
  		display: inline-block;
  		margin-block-start: 0;
  		margin-inline-start: 0;
  		margin-inline-end: 0;
  		margin-block-end: var(--agnostic-input-label-pad, 0.375rem);
  		vertical-align: initial;
  		border-radius: var(--agnostic-radius, 0.25rem);
	}

	.is-active {
		background-color: rgba(143, 207, 253, 0.5);
		border: 4px dashed black;
	}
</style>
