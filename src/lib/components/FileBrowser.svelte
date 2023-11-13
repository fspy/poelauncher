<script lang="ts">
	let fileInput: HTMLInputElement;
	export let filePath = '';
	export let pathMsg = '';

	function openFilePicker() {
		fileInput.click();
	}

	function handleFileChange(event: Event): void {
		const file = (event.target as HTMLInputElement).files?.[0];
		if (file) {
			filePath = file.name;
		}
	}
</script>

<input type="file" bind:this={fileInput} on:change={handleFileChange} class="hidden" />
<label class="block text-emerald-400 font-bold mb-1 pr-4" for="path">Path</label>
<div class="flex gap-2">
	<input
		type="text"
		class="form-input {pathMsg ? '!border-red-500' : 'border-emerald-800'} "
		id="path"
		bind:value={filePath}
	/>
	<button class="button-green" on:click={openFilePicker}>Browse...</button>
</div>
<p class="form-validation">{pathMsg}</p>
