<script lang="ts">
	export let onSubmit = (command: string) => {};

	let value = '';
	let submitted = false;

	function myFunction() {
		onSubmit(value);
		submitted = true;
	}

	function init(element: HTMLInputElement) {
		element.focus();
	}
</script>

<form on:submit|preventDefault={myFunction}>
	{#if submitted}
		<p
			class="ml-2 w-full bg-transparent text-white
                   outline-none"
		>
			{value}
		</p>
	{:else}
		<input
			bind:value
			use:init
			class="ml-2 w-full bg-transparent text-white
                   outline-none"
		/>
	{/if}
</form>
