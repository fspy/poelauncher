import { writable } from 'svelte/store';
export type LauncherItem = {
	id?: number;
	name: string;
	path: string;
	args?: string;
	enabled: boolean;
};

export const selectedLauncher = writable<LauncherItem>({
	enabled: false,
	name: '',
	path: ''
});

export const launchers = writable<LauncherItem[]>([
	{
		id: 1,
		name: 'Poe Lurker',
		path: String.raw`D:\Path of Exile\Tools\PoeLurker.exe`,
		args: '-',
		enabled: false
	},
	{
		id: 2,
		name: 'Awakened PoE Trade',
		path: String.raw`D:\Path of Exile\Tools\Awakened PoE Trade.exe`,
		args: '-',
		enabled: false
	},
	{
		id: 3,
		name: 'Chrome PoE',
		path: String.raw`C:\Program Files (x86)\Google\Chrome\Application\chrome.exe`,
		args: '--profile-directory="Profile 1" --app-id=ebpnegaeffgohjibhmgnhacmfpkhecdk',
		enabled: false
	}
]);

export const showForm = writable(false);
