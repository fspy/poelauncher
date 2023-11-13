<script lang="ts">
	import { launchers, selectedLauncher, showForm, type LauncherItem } from '$lib/store';
	import FileBrowser from './FileBrowser.svelte';
	import DeleteIcon from './icons/Delete.svelte';

	showForm.subscribe((value) => showForm.set(value));

	let nameMsg = '';
	let pathMsg = '';

	function validateInputs() {
		nameMsg = '';
		pathMsg = '';

		if (!$selectedLauncher.name) {
			nameMsg = 'Name is required.';
		}

		const pathRegex =
			/^[a-zA-Z]:\\(?:[^\\/:*?"<>|\r\n]+\\)*[^\\/:*?"<>|\r\n]*\.[^\\/:*?"<>|\r\n]+$/;
		if (!$selectedLauncher.path || !pathRegex.test($selectedLauncher.path)) {
			pathMsg = 'Invalid file path.';
		}

		return !nameMsg && !pathMsg;
	}

	function save() {
		if (validateInputs()) {
			$launchers = $launchers.map((launcher) =>
				launcher.id === $selectedLauncher.id ? $selectedLauncher : launcher
			);

			if (!$launchers.find((launcher) => launcher.id === $selectedLauncher.id)) {
				$launchers = [...$launchers, $selectedLauncher];
			}
			selectedLauncher.set({} as LauncherItem);

			showForm.set(false);

			console.log('we did submit', showForm);
			console.log($selectedLauncher);
		}
	}

	function cancel() {
		showForm.set(false);
	}

	function deleteLauncher(event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }) {
		throw new Error('Function not implemented.');
	}
</script>

<div
	class="p-6 mx-auto bg-zinc-800 rounded flex items-center justify-center flex-col relative outline outline-2 outline-emerald-400"
>
	<h2 class="text-3xl font-bold text-emerald-400 pb-4">Launcher Editor</h2>
	<button class="absolute top-2 right-2 text-zinc-400 p-2 text-xl" on:click={cancel}>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
			class="h-6 w-6"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M6 18L18 6M6 6l12 12"
			/>
		</svg>
	</button>
	<form class="w-[600px]">
		<div class="mb-4">
			<label class="block text-emerald-400 font-bold mb-1 pr-4" for="name"> Name </label>
			<input
				class="form-input {nameMsg ? '!border-red-500' : 'border-emerald-800'}"
				id="name"
				type="text"
				bind:value={$selectedLauncher.name}
			/>
			<p class="form-validation">{nameMsg}</p>
		</div>
		<div class="mb-4">
			<FileBrowser bind:filePath={$selectedLauncher.path} bind:pathMsg />
		</div>
		<div class="mb-8">
			<label class="block text-emerald-400 font-bold mb-1 pr-4" for="name">
				Arguments <small class="pl-1 font-normal text-zinc-500">(optional)</small>
			</label>
			<input class="form-input" id="name" type="text" bind:value={$selectedLauncher.args} />
		</div>
		<div class="flex items-center justify-between">
			<button class="text-zinc-600 hover:text-red-500" on:click={deleteLauncher}>
				<DeleteIcon />
			</button>
			<button class="button-green" type="button" on:click={save}> Save </button>
		</div>
	</form>
</div>
