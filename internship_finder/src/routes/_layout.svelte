<script>
	import Nav from '../components/Nav.svelte';

	// export let segment;
</script>

<style>
	main {
		/* position: relative;
		max-width: 56em;
		background-color: white;
		padding: 2em;
		margin: 0 auto;
		box-sizing: border-box;
		margin-left: 100px;
		padding: 0px 10px; */
		width: 100vw;
		height: 100vh;
	}
</style>

<!-- <Nav {segment}/> -->

<main>
	<slot></slot>
</main>