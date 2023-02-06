<script lang="ts">
	import { Visibility, VisibilityOptions } from '$lib/types';
	import { ChoiceInput } from 'agnostic-svelte';

	export let visibility: Visibility;

	let visibilityCheckedOption: Array<string> = [visibility as string];
	$: visibility = Visibility[visibilityCheckedOption[0] as Visibility];
</script>

<ChoiceInput
	id="visibility"
	type="radio"
	options={VisibilityOptions}
	legendLabel="Visibility"
	checkedOptions={visibilityCheckedOption}
	bind:checked={visibilityCheckedOption}
/>
