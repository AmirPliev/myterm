<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';

	let userName = 'user';
	let currentDir = '/';

	onMount(async () => {
		userName = await invoke('get_username');
		currentDir = await invoke('get_current_directory');
	});
</script>

<div class="flex gap-2 ml-2">
	<p class="text-accent-0">{userName}</p>
	<p class="text-accent-1">&gt;&gt;</p>
	<p class="text-accent-0">{currentDir}:</p>
</div>
