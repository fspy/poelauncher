<script lang="ts">
	import LauncherForm from '$lib/components/LauncherForm.svelte';
	import Table from '$lib/components/Table.svelte';
	import type { LauncherItem } from '$lib/store';
	import { selectedLauncher, showForm, launchers } from '$lib/store';

	function toggleForm() {
		selectedLauncher.set({} as LauncherItem);
		showForm.set(!$showForm);
	}

	function updateShowForm(event: CustomEvent<boolean>): void {
		showForm.set(event.detail);
	}
</script>

<section class="flex justify-center items-center h-screen">
	<div class="w-[900px] aspect-video bg-zinc-900 p-2 ${$showForm ? 'filter blur-sm' : ''}">
		<Table launchers={$launchers} />
		<button class="rounded" on:click={toggleForm}>
			<p>Add</p>
		</button>
	</div>
	{#if $showForm}
		<div class="absolute inset-0 bg-black bg-opacity-25 flex justify-center items-center">
			<LauncherForm on:update={updateShowForm} />
		</div>
	{/if}
</section>
