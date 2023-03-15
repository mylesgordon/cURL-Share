<script lang="ts">
	import { isLoggedIn } from '$lib/stores';
	import IconLink from '$lib/components/IconLink.svelte';
	import Meta from '$lib/components/Meta.svelte';
	import Plus from 'virtual:icons/fa/plus';
	import ProjectListEntry from '$lib/components/ProjectListEntry.svelte';
	import Search from '$lib/components/Search.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
	let filteredProjects = data.data;
</script>

<Meta title="Home" />

<div class="flex flex-row gap-2 justify-between items-center">
	<div class="flex gap-2 items-center">
		<h2>Projects</h2>
		{#if $isLoggedIn}
			<IconLink description="Create project" href="/project/create">
				<Plus />
			</IconLink>
		{/if}
	</div>
	<Search bind:projectInfoArray={filteredProjects} />
</div>

{#if filteredProjects.length}
	{#each filteredProjects as entry (entry.id)}
		<ProjectListEntry {...entry} />
	{/each}
{:else}
	<p>No projects available</p>
{/if}
