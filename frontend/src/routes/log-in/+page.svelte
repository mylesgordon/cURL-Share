<script lang="ts">
	import { Button, ButtonGroup, Input } from 'agnostic-svelte';
	import { goto } from '$app/navigation';
	import { backendUrl, isLoggedIn } from '$lib/stores';
	import Meta from '$lib/components/Meta.svelte';

	let errorText: string;
	let username: string;
	let password: string;

	async function onSubmit(e: SubmitEvent) {
		let endpoint = '';
		switch (e.submitter?.textContent) {
			case 'Log In':
				endpoint = 'log-in';
				break;
			case 'Sign Up':
				endpoint = 'sign-up';
				break;
			default:
				console.error('Submitter not identified.');
				break;
		}

		try {
			let request = await fetch(`${$backendUrl}/api/v1/${endpoint}`, {
				method: 'POST',
				mode: 'cors',
				headers: {
					'Access-Control-Allow-Origin': $backendUrl,
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ username, password }),
				credentials: 'include'
			});

			switch (request.status) {
				case 200:
				case 201:
					isLoggedIn.set(true);
					await goto('/');
					break;
				case 401:
					errorText = 'Incorrect username and/or password - please try again.';
					break;
				case 409:
					errorText = 'User with that username already exists - please try again.';
					break;
				default:
					errorText = 'Unknown error - please try again.';
					break;
			}
		} catch (err) {
			// FIXME
		}
	}
</script>

<Meta title="Log In" />

<form class="flex flex-col space-y-3" on:submit|preventDefault={onSubmit}>
	<Input isRounded id="username" label="Username" bind:value={username} />
	<Input isRounded id="password" type="password" label="Password" bind:value={password} />

	{#if errorText}
		<p id="error-text">{errorText}</p>
	{/if}

	<ButtonGroup ariaLabel="Form submission buttons" css="self-center">
		<Button isGrouped isBordered mode="primary" type="submit">Log In</Button>
		<Button isGrouped isBordered mode="primary" type="submit">Sign Up</Button>
	</ButtonGroup>
</form>
