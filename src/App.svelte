<script lang="ts">
	import Title from './lib/Title.svelte'
	import Projects from './lib/Projects.svelte'
  import Menu from './lib/Menu.svelte'

  document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<main>
	<Title />
	<Projects />
	<Menu />

	<div id="choose-text">
		<p>Choose a project</p>
	</div>
</main>

<style>
	#choose-text {
		position: fixed;
		top: 300px;
		left: 325px;

		font-family: Trebuchet MS;
		font-size: 50px;
		color: #808080;
	}
</style>
