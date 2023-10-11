<script lang="ts">
	import {invoke} from "@tauri-apps/api"


	export let pid: string

	let project: HTMLDivElement | null

	let menu = [
		document.getElementById('menu'),
		document.getElementById('menu-id'),
		document.getElementById('menu-title'),
		document.getElementById('menu-description'),
		document.getElementById('menu-count')
	]

	// on create
	invoke('get', {a: "project", pid: pid, tid: ""})
		.then((responce) => {
			const data = JSON.parse(responce) // project by pid

			// set display name for project in list
			project.children.item(1).textContent = data.title
	})

	// on click project in list
	const clicked = () => {
		document.getElementById('choose-text').style.display = "none" // hide text
		menu[0].style.display = "block" // and display menu

		// got data about projects
		invoke('get', {a: "project", pid: pid, tid: ""})
			.then((responce) => {
				const data = JSON.parse(responce) // project by pid

				// and setting in menu all info about project
				menu[1].textContent = data.id
				menu[2].children.item(0).value = data.title
				menu[3].children.item(0).value = data.description
				menu[4].children.item(0).textContent = data.count.replace('/', ' / ')
		});

		// also display tasks panel whith tasks in this project
		document.querySelectorAll('#tasks').forEach((ul) => {
			if (pid == ul.children.item(0).textContent) // here check text(id) in <p>
				ul.style.display = "block"
			else
				ul.style.display = "none"
		})

		// change color for all others projects which don't current
		document.querySelectorAll('#project').forEach((p) => {
			if (pid == p.children.item(0).textContent)
				p.style.backgroundColor = "#2F2F2F"
			else
				p.style.backgroundColor = "#4D4D4D"
		})
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div bind:this={project} id="project" on:click={clicked}>
	<p id="project-id">{pid}</p>

	<p id="project-title"></p>
</div>

<style>
	#project {
		position: relative;
		top: -15px;
		left: -25px;
		width: 200px;
		height: 25px;
		margin-bottom: -5px;

		background-color: #4D4D4D;
		border-radius: 10px;
		cursor: pointer;
	}

	#project:hover {
		outline: #333333 solid 2px;
	}

	#project-id {
		display: none;
	}

	#project-title {
		position: relative;
		top: 3px;
		left: 10px;
		width: 150px;

		font-size: 16px;
		font-family: Trebuchet MS;
		text-align: left;
		color: #D9D9D9;
	}
</style>