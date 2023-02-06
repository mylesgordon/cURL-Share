<script lang="ts">
	import { ChoiceInput } from 'agnostic-svelte';
	import { Visibility, VisibilityOptions } from '$lib/types';

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
