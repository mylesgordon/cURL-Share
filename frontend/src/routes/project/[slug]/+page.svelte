<script lang="ts">
	import CurlGroupListEntry from '$lib/components/CurlGroupListEntry.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import type { PageData } from './$types';

	import IconLink from '$lib/components/IconLink.svelte';
	import Settings from 'virtual:icons/fa/gear';

	export let data: PageData;
	const { project } = data;
</script>

{#if project}
	<div class="flex flex-row justify-between">
		<div class="flex flex-row gap-2">
			<h1>{project.info.name}</h1>
			<IconLink description="Project settings" href={`/project/${project.info.id}/settings`}>
				<Settings />
			</IconLink>
		</div>
	</div>

	<Meta title={project.info.name} />

	{#if project.groups}
		{#each project.groups as group (group.id)}
			<CurlGroupListEntry {...group} />
		{/each}
	{:else}
		<p>No groups available</p>
	{/if}
{/if}

