<script lang="ts">
	import CurlGroupListEntry from '$lib/components/CurlGroupListEntry.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<Meta title={data.projectName} />

<h1>{data.projectName}</h1>

{#if data.groups.length}
	{#each data.groups as group (group.groupId)}
		<CurlGroupListEntry {...group} projectId={data.projectId} />
	{/each}
{:else}
	<p>No groups available</p>
{/if}
