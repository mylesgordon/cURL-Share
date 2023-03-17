<script lang="ts">
	import { Button, Disclose } from 'agnostic-svelte';
	import AgnosticInput from '$lib/components/AgnosticInput.svelte';
	import Copy from 'virtual:icons/fa/copy';
	import Edit from 'virtual:icons/fa/edit';
	import IconLink from '$lib/components/IconLink.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import type { Curl } from '$lib/types';
	import type { PageData } from './$types';

	export let data: PageData;
	const { curlGroup, project, projectAdminStatus, success } = data;
	const originalCurls: Array<Curl> = JSON.parse(curlGroup.curls);
	let curls = [...originalCurls];

	let chosenEnvironment: string;
	const environments = project?.info.environments ? project.info.environments.split(',') : [];

	$: chosenEnvironment, replaceCurlsWithEnvironment();

	function copyToClipboard(query: string) {
		navigator.clipboard.writeText(query).catch((e) => console.log);
	}

	function replaceCurlsWithEnvironment() {
		if (!chosenEnvironment) {
			return;
		}

		for (let i = 0; i < curls.length; i++) {
			for (const environment of environments) {
				if (originalCurls[i].rawQuery.includes(environment)) {
					curls[i].rawQuery = originalCurls[i].rawQuery.replaceAll(environment, chosenEnvironment);
					break;
				}
			}
		}
	}
</script>

{#if success}
	<Meta title={`Edit ${curlGroup.name}`} />

	<div class="flex space-x-2 items-center">
		<h2>{curlGroup.name}</h2>
		{#if projectAdminStatus.isUserAdmin}
			<IconLink
				description={`Edit ${curlGroup.name}`}
				href={`/project/${curlGroup.project_id}/group/${curlGroup.id}/edit`}
			>
				<Edit />
			</IconLink>
		{/if}
	</div>
	<h3 class="mt-2 mb-4">{curlGroup.description}</h3>

	{#if environments.length}
		<label for="environments">Environments:</label>
		<select name="environments" id="environments" bind:value={chosenEnvironment}>
			<option value="">Choose an environment</option>
			{#each environments as environment}
				<option value={environment}>{environment}</option>
			{/each}
		</select>
	{/if}

	{#each curls as curl}
		<Disclose isBackground isBordered title={curl.name}>
			<section class="flex items-end mb-4">
				<AgnosticInput
					id="raw-query"
					label="Raw cURL query:"
					value={curl.rawQuery}
					readonly
					aria-readonly="true"
				/>
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
