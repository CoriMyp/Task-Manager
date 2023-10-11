<script lang="ts">
	import Project from "./Project.svelte"
	import Tasks from "./Tasks.svelte"

	import {invoke} from "@tauri-apps/api"


	let list: HTMLUListElement | null
	let btn: HTMLButtonElement | null

	// on create
	invoke('get', {a: "projects", pid: "", tid: ""})
		.then((responce) => {
			const data = JSON.parse(responce) // all projects

			// creating list's elements
			data.projects.forEach((p) => {
				var li = document.createElement('li')

				// creating Project
				new Project({
					target: li,
					props: {
						pid: p.id
					}
				})

				// adding li to projects list
				list?.appendChild(li)
			})
	})

	// on button clicked
	const clicked = () => {
		var li = document.createElement('li')

		// creating new project and got data about it
		invoke('create', {a: "project", pid: ""}) // create project
			.then((responce) => { // responce contain only project
				const pid = JSON.parse(responce).id // id of new project

				// new element of Project
				new Project({
					target: li,
					props: {
						pid: pid
					}
				})

				// and Tasks which hidden for now
				new Tasks({
					target: document.querySelector('main'),
					props: {
						pid: pid
					}
				})
		})

		// and adding li to list
		list?.appendChild(li)

		// button animation
		btn?.animate([
			{transform: "scale(0.9)"},
			{transform: "scale(1)"}
		], {duration: 100})
	}
</script>

<div id="projects">
	<p>Projects</p>

	<button bind:this={btn} on:click={clicked}>+</button>

	<hr>

	<ul bind:this={list} id="projects-list"></ul>
</div>

<style>
	#projects {
		position: fixed;
		top: 75px;
		left: 0px;

		width: 225px;
		height: 725px;
		background-color: #3D3C3C;
	}

	#projects p {
		position: fixed;
		top: 67px;
		left: 15px;

		font-size: 20px;
		font-family: Trebuchet MS;
		color: #D9D9D9;
	}

	#projects button {
		position: fixed;
		top: 88px;
		left: 190px;
		width: 25px;
		height: 25px;

		background-color: #3D3C3C;
		border-radius: 5px;
		border: none;
		cursor: pointer;

		color: #D9D9D9;
		font-size: 25px;
	}

	#projects button:hover {
		color: #898989;
	}

	#projects hr {
		position: fixed;
		top: 110px;
		left: 15px;
		width: 200px;

		border: 1px solid #898989;
	}

	#projects ul {
		position: fixed;
		top: 115px;
		left: 0px;
		width: 185px;
		height: 670px;

		list-style: none;

		overflow-x: hidden;
		overflow-y: scroll;
	}

	::-webkit-scrollbar {
		width: 5px;
	}

	::-webkit-scrollbar-thumb {
			background: #4D4D4D; 
			border-radius: 5px;
	}

	::-webkit-scrollbar-thumb:hover {
			background: #555555; 
	}
</style>