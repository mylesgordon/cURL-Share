<script lang="ts">
	import { Button, ButtonGroup, Input } from 'agnostic-svelte';
	import Meta from '$lib/components/Meta.svelte';
	let username: string;
	let password: string;

	async function onSubmit(_e: SubmitEvent) {
		await fetch('http://localhost:8080/api/v1/log-in', {
			method: 'POST',
			mode: 'cors',
			headers: { 'Access-Control-Allow-Origin': '*', 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password })
		});
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
