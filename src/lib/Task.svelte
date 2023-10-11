<script lang="ts">
	import { invoke } from "@tauri-apps/api"


	export let pid: string
	export let tid: string

	let title: HTMLInputElement | null
	let description: HTMLTextAreaElement | null

	let done_btn: HTMLDivElement | null
	let text_btn: HTMLButtonElement | null
	let del_btn: HTMLButtonElement | null

	let task_done = false
	let del_confirm = false

	// on create
	invoke('get', {a: "task", pid: pid, tid: tid})
		.then((responce) => {
			const data = JSON.parse(responce) // task from project

			// init task
			title.value = data.title
			description.value = data.description

			if (data.done) {
				task_done = true
				done_btn.style.backgroundColor = "#338E26"
			}
	})

	// on title focus lost
	const tblur = () => {
		// just updating title in file
		invoke('update', { a: "task",
			pid: pid,
			tid: tid,
			param: 'title',
			value: title?.value
		})
	}

	// on description key pressed
	const dpress = (e) => {
		if (e.keyCode == 13) return false
	}

	// on description focus lost
	const dblur = () => {
		// and updating description in file too
		invoke('update', { a: "task",
			pid: pid,
			tid: tid,
			param: 'description',
			value: description?.value
		})
	}

	// on click done button(div)
	const doclicked = () => {
		// updating tasks which done in menu
		var count = document.getElementById('menu-count')
		var count_str = count?.textContent?.replaceAll(' ', '').split('/')
		var count_nums = [Number.parseInt(count_str[0]), Number.parseInt(count_str[1])]

		// if done, do undone and if undone - do done
		if (task_done) {
			count_nums[0] -= 1
			done_btn.style.backgroundColor = "#646464"
		} else {
			count_nums[0] += 1
			done_btn.style.backgroundColor = "#338E26"
		}

		// reverse value and set count in menu
		task_done = !task_done
		count.children.item(0).textContent = count_nums[0] + ' / ' + count_nums[1]

		// updating "done" for task
		invoke('update', { a: "task",
			pid: pid,
			tid: tid,
			param: 'done',
			value: task_done.toString()
		})

		// updating tasks done for project
		invoke('update', { a: "project",
			pid: pid,
			tid: "",
			param: 'count',
			value: count_nums[0] + '/' + count_nums[1]
		})

		// animate button
		done_btn?.animate([
			{transform: "scale(1.1)"},
			{transform: "scale(1)"}
		], {duration: 100})
	}

	// on click text button (show description)
	const tclicked = () => {
		var ddiv = description?.parentElement // div of description

		// hide/show description
		if (ddiv?.style.display == "none")
			ddiv.style.display = "block"
		else ddiv.style.display = "none"

		// animate description show
		ddiv?.animate([
			{transform: "translateX(100px)"},
			{transform: "translateX(0px)"}
		], {duration: 100})

		// animate button
		text_btn?.animate([
			{transform: "scale(0.9)"},
			{transform: "scale(1)"}
		], {duration: 100})
	}

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

		// deleting task on frontend
		done_btn?.parentElement?.remove()

		// updating counts in menu
		var count = document.getElementById('menu-count')
		var count_str = count?.textContent?.replaceAll(' ', '').split('/')
		var count_nums = [Number.parseInt(count_str[0]), Number.parseInt(count_str[1])]

		// if task was done will dec from done tasks
		if (task_done) count_nums[0] -= 1
		count_nums[1] -= 1 // and always will dec from total tasks

		// updating in menu
		count.children.item(0).textContent = count_nums[0] + ' / ' + count_nums[1]

		// updating count tasks in menu
		invoke('update', { a: "project",
			pid: pid,
			tid: "",
			param: 'count',
			value: count_nums[0] + '/' + count_nums[1]
		})

		// and delete this task
		invoke('delete', {a: "task", pid: pid, tid: tid})

		// button animate
		del_btn?.animate([
			{transform: "scale(0.9)"},
			{transform: "scale(1)"}
		], {duration: 100})
	}
</script>

<div id="task">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div bind:this={done_btn} id="task-done" on:click={doclicked}></div>

	<div id="task-frame">
		<input bind:this={title} id="task-title" on:blur={tblur}>

		<button bind:this={text_btn} id="btn-text" on:click={tclicked}>...</button>

		<button bind:this={del_btn} id="btn-del" on:click={dclicked}>X</button>
	</div>

	<div id="task-description" style="display: none;">
		<textarea bind:this={description} on:keypress={dpress} on:blur={dblur}></textarea>
	</div>
</div>

<style>
	#task {
		position: relative;
		top: 0px;
		left: -40px;
		margin-bottom: -20px;
	}

	#task-done {
		position: relative;
		top: 10px;
		left: 0px;
		width: 30px;
		height: 30px;

		background-color: #646464;
		box-sizing: border-box;
		border-radius: 10px;
		cursor: pointer;
	}

	#task-frame {
		position: relative;
		top: -30px;
		left: 40px;
		width: 500px;
		height: 50px;

		background-color: #434343;
		border-radius: 30px;
	}

	#task-title {
		position: relative;
		top: 8px;
		left: 10px;
		width: 425px;
		height: 30px;

		background-color: #434343;
		border: none;
		outline: none;

		color: #BCBCBC;
		font-size: 20px;
		font-family: Trebuchet MS;
		text-align: left;
	}

	#btn-text {
		position: relative;
		top: 18px;
		left: 23px;
		width: 30px;
		height: 20px;

		background-color: #333333;
		border: none;
		border-radius: 5px;
		cursor: pointer;

		color: #BCBCBC;
		font-size: 15px;
		font-family: Trebuchet MS;
		text-align: center;
	}

	#btn-text:hover {
		color: white;
	}

	#btn-del {
		position: relative;
		top: -7px;
		left: -1px;
		width: 20px;
		height: 20px;

		background-color: #333333;
		border: none;
		border-radius: 5px;
		cursor: pointer;

		color: #F44336;
		font-size: 10px;
		font-family: Trebuchet MS;
		text-align: center;
	}

	#btn-del:hover {
		color: #898989;
	}

	#task-description {
		position: relative;
		top: -29px;
		left: 65px;
		width: 420px;
		height: 90px;

		background-color: #333333;
		border-bottom-right-radius: 10px;
		border-bottom-left-radius: 10px;
	}

	#task-description textarea {
		position: relative;
		top: 0px;
		left: 5px;
		width: 408px;
		height: 83px;

		background-color: #333333;
		border: none;
		outline: none;
		resize: none;
		overflow-y: hidden;

		font-size: 12px;
		font-family: Arial;
		color: #C1C1C1;
		text-align: left;
	}
</style>