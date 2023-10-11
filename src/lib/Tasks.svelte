<script lang="ts">
	import Task from "./Task.svelte"

	import { invoke } from "@tauri-apps/api"


	export let pid: string

	let tasks: HTMLUListElement | null

	// on create
	invoke('get', {a: "project", pid: pid, tid: ""})
		.then((responce) => {
			const data = JSON.parse(responce) // project

			data.tasks.forEach((t) => { // and got tasks of it
				var li = document.createElement('li')

				new Task({ // creating Tasks in <li>
					target: li,
					props: {
						pid: pid,
						tid: t.id.toString()
					}
				})

				tasks?.appendChild(li) // and adding it
			})
	})
</script>

<ul bind:this={tasks} id="tasks">
	<p id="tasks-id">{pid}</p>
</ul>

<style>
	#tasks {
		position: fixed;
		top: 125px;
		left: 240px;
		height: 660px;
		width: 520px;

		
		list-style: none;
		display: none;

		overflow-x: hidden;
		overflow-y: scroll;
	}

	#tasks-id {
		display: none;
	}

	::-webkit-scrollbar {
		width: 8px;
	}

	::-webkit-scrollbar-thumb {
			background: #3D3C3C; 
			border-radius: 5px;
	}

	::-webkit-scrollbar-thumb:hover {
			background: #444444; 
	}
</style>