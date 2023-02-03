<script lang="ts">
	import type { PageData } from './$types';
	import Meta from '$lib/components/Meta.svelte';
	import ProjectListEntry from '$lib/components/ProjectListEntry.svelte';

	export let data: PageData;
</script>

<Meta title="Home" />

<h2>Projects</h2>

{#if data.entries.length}
	{#each data.entries as entry (entry.projectId)}
		<ProjectListEntry {...entry} />
	{/each}
{:else}
	<p>No projects available</p>
{/if}
