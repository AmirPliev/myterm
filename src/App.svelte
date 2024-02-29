<script lang="ts">
	import Entry from './lib/Entry.svelte';

	let counter = 1;
</script>

<main class="bg-background-main h-screen overflow-hidden py-2 overflow-y-scroll">
	{#each Array.from({ length: counter }) as _, i}
		<Entry nextCommand={() => (counter += 1)} />
	{/each}
</main>
