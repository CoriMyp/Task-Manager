<script lang="ts">
  import Tasks from "./Tasks.svelte"
  import Task from "./Task.svelte"

  import { invoke } from "@tauri-apps/api"

  let mid: HTMLParagraphElement | null
	let title: HTMLInputElement | null
	let description: HTMLTextAreaElement | null
	let count: HTMLParagraphElement | null

	let del_btn: HTMLButtonElement | null
	let add_btn: HTMLButtonElement | null

	// on create
	invoke('get', {a: "projects", pid: "", tid: ""})
		.then((responce) => {
			const data = JSON.parse(responce) // all projects

			// creating Tasks panels for all projects (they hiiden by default)
			data.projects.forEach((p) => {
				new Tasks({
					target: document.querySelector('main'),
					props: {
						pid: p.id
					}
				})
			})
	})

	// on input in title
	const tinput = () => {
		// searching loaded project in list of projects
		document.querySelectorAll('#project').forEach((p) => {
			var pid = p.children.item(0).textContent // project id

			// live setting project name in list of projects
			if (pid == mid?.textContent) {
				p.children.item(1).textContent = title?.value
			}
		})
	}

	// on title focus lost
	const tblur = () => {
		// updating project title when title lost focus
		invoke('update', { a: "project",
			pid: mid?.textContent,
			tid: "",
			param: 'title',
			value: title?.value
		})
	}


	// on keypress in description
	const dpress = (e) => {
		if (e.keyCode == 13) return false
	}

	// on description focus lost
	const dblur = () => {
		// updating description of project on focus lost
		invoke('update', { a: "project",
			pid: mid?.textContent,
			tid: "",
			param: 'description',
			value: description?.value
		})
	}

	// event for buttons
	let del_confirm = false

	// on clicked delete button
	const dclicked = () => {
		if (!del_confirm) {
			del_btn.style.color = "#898989"
			del_confirm = true

			setTimeout(() => {
				del_btn.style.color = "#F44336"
				del_confirm = false
			}, 1000)

			return
		}

		del_btn.style.color = "#F44336"
		del_confirm = false

		// bring back "Choose the Project" text and hide menu
		document.getElementById('choose-text').style.display = "block"
		document.getElementById('menu').style.display = "none"

		// delete task panel
		document.querySelectorAll('#tasks').forEach((t) => {
			if (t.children.item(0).textContent == mid?.textContent) {
				t.remove()
			}
		})

		// and delete project in list
		document.querySelectorAll('#project').forEach((p) => {
			if (p.children.item(0).textContent == mid?.textContent) {
				p.parentElement.remove()
			}
		})

		// delete the project from file
		invoke('delete', {a: "project",
			pid: mid?.textContent,
			tid: ""
		})

		// animate button
		del_btn.animate([
			{transform: "scale(0.9)"},
			{transform: "scale(1)"}
		], {duration: 100})
	}

	// on clicked add button
	const aclicked = () => {
		document.querySelectorAll('#tasks').forEach((t) => {
			if (t.children.item(0).textContent == mid?.textContent) { // searching for tasks of project
				var li = document.createElement('li')

				// creating new task in file
				invoke('create', {a: "task", pid: mid?.textContent})
					.then((responce) => {
						const tid = JSON.parse(responce).id // id of created task

						new Task({ // and creating in frontend
							target: li,
							props: {
								pid: mid?.textContent,
								tid: tid
							}
						})
				})

				t.appendChild(li)
			}
		})

		// updating counts in menu
		var count_str = count.textContent.replaceAll(' ', '').split('/')
		var count_nums = [Number.parseInt(count_str[0]), Number.parseInt(count_str[1])]

		// just adding +1 to total tasks
		count_nums[1] += 1

		// updating on frontend
		count.textContent = count_nums[0] + ' / ' + count_nums[1]

		// and on backend
		invoke('update', { a: "project",
			pid: mid?.textContent,
			tid: "",
			param: 'count',
			value: count_nums[0] + '/' + count_nums[1]
		})

		// animate button
		add_btn.animate([
			{transform: "scale(0.9)"},
			{transform: "scale(1)"}
		], {duration: 100})
	}
</script>

<div id="menu">
	<p bind:this={mid} id="menu-id">ID</p>

	<div id="menu-title">
		<input bind:this={title} on:input={tinput} on:blur={tblur} maxlength="15">

		<button bind:this={del_btn} on:click={dclicked}>X</button>
	</div>

	<div id="menu-description">
		<textarea bind:this={description} on:keypress={dpress} on:blur={dblur} maxlength="150"></textarea>

		<div id="menu-count">
			<p bind:this={count}>0 / 0</p>
		</div>
	</div>

	<div id="menu-add">
		<button bind:this={add_btn} on:click={aclicked}>+</button>
	</div>

	<hr>	
</div>

<style>
	#menu, #menu-id {
		display: none;
	}

	#menu-title {
		position: fixed;
		top: 12px;
		left: 275px;
		width: 500px;
		height: 51px;

		background-color: #3D3C3C;
		border-radius: 50px;
	}

	#menu-title input {
		position: relative;
		top: 10px;
		left: 20px;
		width: 435px;
		height: 30px;
		text-align: center;

		background-color: #3D3C3C;
		border: none;
		outline: none;

		font-size: 24px;
		font-family: Trebuchet MS;
		color: #C9C9C9;
		text-align: center;
	}

	#menu-title button {
		position: relative;
		top: -5px;
		left: 25px;
		width: 20px;
		height: 20px;

		background-color: #333333;
		border: none;
		border-radius: 5px;
		cursor: pointer;

		color: #F44336;
		font-size: 10px;
	}

	#menu-title button:hover {
		color: #898989;
	}

	#menu-description {
		position: fixed;
		top: 67px;
		left: 275px;
		width: 440px;
		height: 50px;

		background-color: #3D3C3C;
		border-radius: 50px;
	}

	#menu-description textarea {
		position: relative;
		top: 2px;
		left: 15px;
		width: 325px;
		height: 40px;

		background-color: #3D3C3C;
		border: none;
		outline: none;
		resize: none;
		overflow-y: hidden;

		font-size: 12px;
		font-family: Arial;
		color: #C1C1C1;
		text-align: left;
	}

	#menu-count {
		position: fixed;
		top: 77px;
		left: 625px;
		width: 80px;
		height: 30px;

		background-color: #333333;
		border-radius: 10px;
	}

	#menu-count p {
		position: relative;
		top: -17px;
		left: 10px;
		width: 60px;
		height: 22px;
		text-align: center;

		font-size: 20px;
		font-family: Trebuchet MS;
		color: #C1C1C1;
	}

	#menu-add {
		position: relative;
		top: 59px;
		left: 715px;
		width: 50px;
		height: 50px;

		background-color: #3D3C3C;
		border-radius: 20px;
	}

	#menu-add button {
		position: relative;
		top: 10px;
		left: 10px;
		width: 30px;
		height: 30px;

		font-size: 25px;
		font-display: block;
		color: green;
		background-color: #333333;
		border: none;
		border-radius: 10px;
		cursor: pointer;
	}

	#menu-add button:hover {
		color: #898989;
	}

	hr {
		position: fixed;
		top: 120px;
		left: 275px;
		width: 500px;

		border-color: #898989;
	}
</style>