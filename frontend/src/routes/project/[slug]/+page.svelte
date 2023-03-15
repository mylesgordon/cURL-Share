<script lang="ts">
	import CurlGroupListEntry from '$lib/components/CurlGroupListEntry.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import type { PageData } from './$types';

	import IconLink from '$lib/components/IconLink.svelte';
	import Plus from 'virtual:icons/fa/plus';
	import Search from '$lib/components/Search.svelte';
	import Settings from 'virtual:icons/fa/gear';
	import type { CurlGroup } from '$lib/types';

	export let data: PageData;
	const { adminStatus, project } = data;

	let filteredGroups: CurlGroup[] = project?.groups ?? [];
</script>

{#if project}
	<div class="flex flex-row gap-2 justify-between items-center">
		<div class="flex gap-2 items-center">
			<h1>{project.info.name}</h1>
			{#if adminStatus.isUserAdmin}
				<IconLink description="Create project" href={`/project/${project.info.id}/group/create`}>
					<Plus />
				</IconLink>
				<IconLink description="Project settings" href={`/project/${project.info.id}/settings`}>
					<Settings />
				</IconLink>
			{/if}
		</div>
		<Search bind:curlGroupArray={filteredGroups} />
	</div>

	<Meta title={project.info.name} />

	{#if project.groups}
		{#each filteredGroups as group (group.id)}
			<CurlGroupListEntry {...group} />
		{/each}
	{:else}
		<p>No groups available</p>
	{/if}
{:else}
	<p>
		Something went wrong loading this project - please ensure you have permission and try again.
	</p>
{/if}
