<script lang="ts">
	import { Button, ButtonGroup, Input } from 'agnostic-svelte';
	import { goto } from '$app/navigation';
	import { isLoggedIn } from '$lib/stores';
	import Meta from '$lib/components/Meta.svelte';

	let username: string;
	let password: string;

	async function onSubmit(e: SubmitEvent) {
		let endpoint = "";
		switch (e.submitter?.textContent) {
			case "Log In":
				endpoint = "log-in";
				break;
			case "Sign Up":
				endpoint = "sign-up";
				break;
			default:
				console.error("Submitter not identified.");
				break;
		};

		try {
			await fetch(`http://localhost:8080/api/v1/${endpoint}`, {
				method: 'POST',
				mode: 'cors',
				headers: {
					'Access-Control-Allow-Origin': 'http://localhost:8080',
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ username, password }),
				credentials: 'include'
			});
			isLoggedIn.set(true);
			await goto('/');
		} catch (err) {
			// FIXME
		}
	}
</script>

<Meta title="Log In" />

<form class="flex flex-col space-y-3" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="username" label="Username" bind:value={username} />
	<Input isRounded id="password" type="password" label="Password" bind:value={password} />

	<ButtonGroup ariaLabel="Form submission buttons" css="self-center">
		<Button isGrouped isBordered mode="primary" type="submit">Log In</Button>
		<Button isGrouped isBordered mode="primary" type="submit">Sign Up</Button>
	</ButtonGroup>
</form>
