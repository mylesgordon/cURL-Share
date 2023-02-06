<script lang="ts">
	import CurlGroupListEntry from '$lib/components/CurlGroupListEntry.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import type { PageData } from './$types';

	import IconLink from '$lib/components/IconLink.svelte';
	import Settings from 'virtual:icons/fa/gear';

	export let data: PageData;
</script>

<div class="flex flex-row justify-between">
	<div class="flex flex-row gap-2">
		<h1>{data.projectName}</h1>
		<IconLink description="Project settings" href={`/project/${data.projectId}/settings`}>
			<Settings />
		</IconLink>
	</div>
</div>

<Meta title={data.projectName} />

{#if data.groups.length}
	{#each data.groups as group (group.groupId)}
		<CurlGroupListEntry {...group} projectId={data.projectId} />
	{/each}
{:else}
	<p>No groups available</p>
{/if}
