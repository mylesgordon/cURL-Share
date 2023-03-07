<script lang="ts">
	import IconLink from '$lib/components/IconLink.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import Plus from 'virtual:icons/fa/plus';
	import ProjectListEntry from '$lib/components/ProjectListEntry.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<Meta title="Home" />

<div class="flex flex-row gap-2">
	<h2>Projects</h2>
	<IconLink description="Create project" href="/project/create">
		<Plus />
	</IconLink>
</div>

{#if data.data.length}
	{#each data.data as entry (entry.id)}
		<ProjectListEntry {...entry} />
	{/each}
{:else}
	<p>No projects available</p>
{/if}
