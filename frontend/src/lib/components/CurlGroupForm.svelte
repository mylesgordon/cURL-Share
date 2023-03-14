<script lang="ts">
	import { Button } from 'agnostic-svelte';
	import { createCurlGroupRequest } from '$lib/api.page';
	import { flip } from 'svelte/animate';
	import { goto } from '$app/navigation';
	import AgnosticInput from './AgnosticInput.svelte';
	import Arrows from 'virtual:icons/fa/arrows';
	import Input from 'agnostic-svelte/components/Input/Input.svelte';
	import type { Curl, CurlGroup } from '$lib/types';

	enum ButtonType {
		Up = -1,
		Down = 1
	}

	export let projectId: number;

	let curlList: Array<Curl> = [];
	let description: string;
	let labels: string;
	let name: string;
	let rawCurlInput: string;
	let maxId = Math.max(...curlList.map((data) => data.id), 0);

	async function createCurlGroup(curlGroupObject: CurlGroup) {
		try {
			const { id } = await createCurlGroupRequest(window.fetch, curlGroupObject);
			goto(`/project/${projectId}/group/${id}`);
		} catch (e) {
			console.log(e);
		}
	}

	function onSubmit() {
		const curlGroupObject: CurlGroup = {
			id: -1,
			description,
			labels,
			curls: JSON.stringify(curlList),
			name,
			project_id: projectId
		};
		createCurlGroup(curlGroupObject);
	}

	// cURLs
	let hovering = -1;

	function addCurl() {
		maxId += 1;
		curlList = [
			...curlList,
			{
				id: maxId,
				name: `cURL #${maxId}`,
				description: 'Insert description here',
				rawQuery: rawCurlInput
			}
		];
		rawCurlInput = '';
	}

	function buttonClick(index: number, buttonType: ButtonType) {
		const itemDestination = (index + buttonType) as number;
		if (curlList[itemDestination] === undefined) {
			return;
		}
		swapItems(index, itemDestination);
	}

	function swapItems(a: number, b: number) {
		[curlList[a], curlList[b]] = [curlList[b], curlList[a]];
		curlList = curlList;
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
	<Input isRounded id="labels" label="Labels (comma separated)" bind:value={labels} />

	<span>cURLs</span>
	<!-- TODO: aria-describedby -->

	<ol class="flex flex-col space-y-2">
		{#each curlList as data, index (data.id)}
			<li
				class="flex flex-col space-y-2 border rounded-md p-2"
				class:is-active={hovering === index}
				animate:flip={{ duration: 750 }}
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
				<div class="flex flex-row">
					<div
						aria-hidden="true"
						class="flex items-center pr-2 cursor-grab"
						draggable="true"
						on:dragstart={(event) => dragStart(event, index)}
						on:dragenter|preventDefault
					>
						<Arrows />
					</div>

					<div class="pt-2 pb-2 pr-2 w-full flex flex-col space-y-2">
						<AgnosticInput id="name" label="Name:" bind:value={data.name} />
						<AgnosticInput id="description" label="Description:" bind:value={data.description} />
						<AgnosticInput id="raw-query" label="Raw cURL Query:" bind:value={data.rawQuery} />

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
								isDisabled={index === curlList.length - 1}
							>
								<span class="sr-only">Move {data.name} down</span>
								<svg width="16" height="16" focusable="false" aria-hidden="true">
									<use xlink:href="#icon--down" />
								</svg>
							</Button>
						</div>
					</div>
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

	.is-active {
		background-color: rgba(143, 207, 253, 0.5);
		border: 4px dashed black;
	}
</style>
