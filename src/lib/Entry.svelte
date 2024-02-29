<script lang="ts">
	import Prompt from './Prompt.svelte';
	import Command from './Command.svelte';

	import { invoke } from '@tauri-apps/api';

	export let nextCommand = () => {};

	let outputText = '';

	async function newEntry(command: string) {
		const result = await invoke('perform_command', { command });
		outputText = result as string;
		nextCommand();
	}
</script>

<div class="flex flex-col w-full pr-8 mt-1">
	<div class="flex w-full pr-9">
		<Prompt />

		<div class="grow">
			<Command onSubmit={newEntry} />
		</div>
	</div>

	{#if outputText}
		<p class="ml-2 w-full bg-transparent text-white outline-none">
			{outputText}
		</p>
	{/if}
</div>
