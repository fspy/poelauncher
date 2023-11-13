<script lang="ts">
	import type { LauncherItem } from '$lib/store';
	import { selectedLauncher, showForm } from '$lib/store';
	import DeleteIcon from './icons/Delete.svelte';
	import EditIcon from './icons/Edit.svelte';
	export let launchers: LauncherItem[];

	function handleEditClick(launcher: LauncherItem) {
		selectedLauncher.set(launcher);
		showForm.set(true);
	}

	function handleDeleteClick(launcher: LauncherItem) {
		throw new Error('Function not implemented.');
	}
</script>

<div class="p-4 grid grid-cols-[min-content,1fr,min-content] gap-4 gap-x-6 items-center">
	<div class="font-bold text-zinc-200">Enabled</div>
	<div class="font-bold text-zinc-200">Name</div>
	<div class="font-bold text-zinc-200 text-center">Actions</div>

	<div class="col-span-3 h-0.5 bg-emerald-500 opacity-25" />
	{#each launchers as launcher (launcher.id)}
		<div class="justify-self-center">
			<label class="sr-only" for="row{launcher.id}">item #{launcher.id}</label>
			<input class="h-5 w-5 rounded border-slate-300" type="checkbox" id="row{launcher.id}" />
		</div>
		<div class="text-zinc-200 text-xl">{launcher.name}</div>
		<div class="flex gap-2">
			<button
				class="text-emerald-100 hover:text-emerald-400"
				on:click={() => handleEditClick(launcher)}
			>
				<EditIcon />
			</button>
			<button class="text-red-200 hover:text-red-500" on:click={() => handleDeleteClick(launcher)}>
				<DeleteIcon />
			</button>
		</div>
		<div class="col-span-3 h-px bg-zinc-500 opacity-10" />
	{/each}
</div>
