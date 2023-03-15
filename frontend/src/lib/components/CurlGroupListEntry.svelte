<script lang="ts">
	import { Card, Tag } from 'agnostic-svelte';

	export let index: number;
	export let length: number;

	export let description: string;
	export let id: number;
	export let labels: string;
	export let project_id: number;
	export let name: string;

	const dataTestIdPrefix = `curl-group-list-entry-${id}`;
	const labelsArray = labels.split(',');
	const link = `/project/${project_id}/group/${id}`;
</script>

<Card isBorder isRounded isShadow css="mt-4">
	<a
		href={link}
		class="p-4 flex-grow full truncate text-center project-title"
		data-testid={dataTestIdPrefix + '-title'}
		aria-label={`cURL group ${name}, ${index} of ${length}`}
	>
		{name}
	</a>
	<p
		class="p-4 flex-grow project-description"
		data-testid={dataTestIdPrefix + '-description'}
		aria-label={`Description`}
	>
		{description}
	</p>
	<div
		aria-label="Project labels"
		class="flex flex-grow flex-wrap justify-center project-label-container"
	>
		{#each labelsArray as label}
			<span data-testid={dataTestIdPrefix + '-label-' + label}>
				<Tag shape="pill">{label}</Tag>
			</span>
		{/each}
	</div>
</Card>

<style>
	/* TODO: figure out tailwind ignoring classes */
	.project-title {
		width: 8rem;
	}
	.project-description {
		width: 32rem;
	}
	.project-label-container {
		width: 14rem;
		gap: 0.5rem;
		padding: 1rem 0 1rem 1rem;
	}
</style>
