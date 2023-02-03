<script lang="ts">
	import { Card } from 'agnostic-svelte';

	export let description: string;
	export let link: string;
	export let projectId: number;
	export let title: string;
	const dataTestIdPrefix = `project-list-entry-${projectId}`;
</script>

<Card isBorder isRounded isShadow css="mt-4">
	<a href={link} class="p-4 w-48 truncate" data-testid={dataTestIdPrefix + '-title'}>
		{title}
	</a>
	<p class="p-4 flex-grow flex-shrink" data-testid={dataTestIdPrefix + '-description'}>
		{description}
	</p>
</Card>
