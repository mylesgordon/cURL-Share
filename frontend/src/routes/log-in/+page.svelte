<script lang="ts">
	import Meta from '$lib/components/Meta.svelte';
	import { Button, ButtonGroup, Input } from 'agnostic-svelte';
	let username: string;
	let password: string;

	function onSubmit(e: SubmitEvent) {
		console.log(typeof e);
	}
</script>

<Meta title="Log In" />

<form class="flex flex-col space-y-3" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="username" label="Username" bind:value={username} />
	<Input isRounded id="password" type="password" label="Password" bind:value={password} />

	<ButtonGroup ariaLabel="Form submission buttons" css="self-center">
		<Button isGrouped isBordered mode="primary" type="submit">Log In</Button>
		<Button isGrouped isBordered mode="primary">Sign Up</Button>
	</ButtonGroup>
</form>
