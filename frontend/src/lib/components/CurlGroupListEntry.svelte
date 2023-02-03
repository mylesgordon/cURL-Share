<script lang="ts">
	import { Card, Tag } from 'agnostic-svelte';

	export let description: string;
	export let groupId: number;
	export let labels: Array<string>;
	export let projectId: string;
	export let title: string;

	const dataTestIdPrefix = `curl-group-list-entry-${groupId}`;
	const link = `/project/${projectId}/group/${groupId}`;
</script>

<Card isBorder isRounded isShadow css="mt-4">
	<a
		href={link}
		class="p-4 flex-grow full truncate text-center project-title"
		data-testid={dataTestIdPrefix + '-title'}
	>
		{title}
	</a>
	<p class="p-4 flex-grow project-description" data-testid={dataTestIdPrefix + '-description'}>
		{description}
	</p>
	<div
		aria-label="Project labels"
		class="flex flex-grow flex-wrap justify-center project-label-container"
	>
		{#each labels as label}
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
