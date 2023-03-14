<script lang="ts">
	import { Button, Disclose } from 'agnostic-svelte';
	import AgnosticInput from '$lib/components/AgnosticInput.svelte';
	import Copy from 'virtual:icons/fa/copy';
	import Edit from 'virtual:icons/fa/edit';
	import IconLink from '$lib/components/IconLink.svelte';
	import type { Curl } from '$lib/types';
	import type { PageData } from './$types';

	export let data: PageData;
	const { curlGroup, success } = data;
	const curls: Array<Curl> = JSON.parse(curlGroup.curls);

	function copyToClipboard(query: string) {
		navigator.clipboard.writeText(query).catch((e) => console.log);
	}
</script>

{#if success}
	<div class="flex space-x-2">
		<h2>{curlGroup.name}</h2>
		<IconLink
			description={`Edit ${curlGroup.name}`}
			href={`/project/${curlGroup.project_id}/group/${curlGroup.id}/edit`}
		>
			<Edit />
		</IconLink>
	</div>
	<h3 class="mt-2 mb-4">{curlGroup.description}</h3>

	{#each curls as curl}
		<Disclose isBackground isBordered title={curl.name}>
			<section class="flex items-end mb-4">
				<AgnosticInput id="raw-query" label="Raw cURL query:" value={curl.rawQuery} readonly />
				<Button
					isRounded
					style="height: 2.375rem"
					on:click={() => {
						copyToClipboard(curl.rawQuery);
					}}
				>
					<span class="sr-only">Copy cURL query</span>
					<Copy aria-hidden="true" />
				</Button>
			</section>

			<h4 class="font-semibold">Description:</h4>
			<p>{curl.description}</p>
		</Disclose>
	{/each}
{:else}
	<p>Failed to load cURL group. Please try again.</p>
{/if}
