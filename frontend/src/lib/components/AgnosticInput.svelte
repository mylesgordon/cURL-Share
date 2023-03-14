<script lang="ts">
	export let id: string;
	export let label: string;
	export let value: string;
</script>

<div class="flex flex-col flex-grow overflow-hidden">
	<label for={id}>{label}</label>
	<input {id} class="input" bind:value {...$$props} />
</div>

<style>
	/* There is an issue with undefined bindings with agnostic ui's svelte implementation.
	This is just a direct copy of the input, input-rounded and label css styles */
	.input {
		color: var(--agnostic-font-color, var(--agnostic-dark));
		font-family: var(--agnostic-font-family-body);
		font-weight: var(--agnostic-font-weight, 300);
		font-size: var(--agnostic-font-size, 1rem);
		line-height: var(--agnostic-line-height, var(--fluid-20, 1.25rem));
		width: 100%;
		max-width: 100%;
		border-style: solid;
		border-width: var(--agnostic-input-border-size, 1px);
		border-color: var(--agnostic-input-border-color, var(--agnostic-gray-light));
		padding-block-start: var(--agnostic-input-vertical-pad, 0.5rem);
		padding-block-end: var(--agnostic-input-vertical-pad, 0.5rem);
		padding-inline-start: var(--agnostic-input-side-padding, 0.75rem);
		padding-inline-end: var(--agnostic-input-side-padding, 0.75rem);
		transition-property: box-shadow;
		transition-duration: var(--agnostic-input-timing, var(--agnostic-timing-medium));
		display: inline-block;
		margin-block-start: 0;
		margin-inline-start: 0;
		margin-inline-end: 0;
		/* margin-block-end: var(--agnostic-input-label-pad, 0.375rem); */
		vertical-align: initial;
		border-radius: var(--agnostic-radius, 0.25rem);
	}
</style>
