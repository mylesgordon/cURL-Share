<script lang="ts">
	import { Card } from 'agnostic-svelte';

	export let index: number;
	export let length: number;

	export let description: string;
	export let id: number;
	export let name: string;

	const dataTestIdPrefix = `project-list-entry-${id}`;
	const link = `/project/${id}`;
</script>

<Card isBorder isRounded isShadow css="mt-4">
	<a
		href={link}
		class="p-4 w-48 truncate"
		data-testid={dataTestIdPrefix + '-title'}
		aria-label="Project name {name}, {index} of {length}"
	>
		{name}
	</a>
	<p
		class="p-4 flex-grow flex-shrink"
		data-testid={dataTestIdPrefix + '-description'}
		aria-label="Description"
	>
		{description}
	</p>
</Card>
