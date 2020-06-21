<script>
	export let segment;
</script>

<style>
	nav {
		font-weight: 300;
		height: 100%;
		width: 100px;
		position: fixed;
		z-index: 1;
		top: 0;
		left: 0;
		background: linear-gradient(#037fff, #1fa6fa);
		overflow-x: hidden;
		padding-top: 20px;
		box-shadow: 5px 0px 20px 0px rgba(100, 100, 100, 0.473);
	}

	ul {
		margin: 0;
		padding: 0;
	}

	/* clearfix */
	ul::after {
		content: '';
		display: block;
		clear: both;
	}

	li {
		margin-bottom: 1em;
		text-align: center;
		/* margin: auto 0; */
		display: block;
		/* float: left; */
	}

	[aria-current] {
		position: relative;
		display: inline-block;
	}

	[aria-current]::after {
		position: absolute;
		content: '';
		width: calc(100% - 1em);
		height: 2px;
		/* background-color: rgb(255,62,0); */
		display: block;
		bottom: -1px;
	}

	a {
		text-decoration: none;
		padding: 1em 0.5em;
		display: block;
	}
</style>

<nav>
	<ul>
		<li>
			<a aria-current="{segment === undefined ? 'page' : undefined}" href=".">
				<img alt='Success Kid' src='search.svg'>
			</a>
		</li>
		<li>
			<a aria-current="{segment === 'map' ? 'page' : undefined}" href="map">
				<img alt='Success Kid' src='map.svg'>
			</a>
		</li>
		<li>
			<a aria-current="{segment === 'profile' ? 'page' : undefined}" href="profile">
				<img alt='Success Kid' src='profile.svg'>
			</a>
		</li>
		<li>
			<a aria-current="{segment === 'settings' ? 'page' : undefined}" href="settings">
				<img alt='Success Kid' src='settings.svg'>
			</a>
		</li>


		<!-- <li><a aria-current="{segment === undefined ? 'page' : undefined}" href=".">home</a></li>
		<li><a aria-current="{segment === 'about' ? 'page' : undefined}" href="about">about</a></li> -->

		<!-- for the blog link, we're using rel=prefetch so that Sapper prefetches
		     the blog data when we hover over the link or tap it on a touchscreen -->
		<!-- <li><a rel=prefetch aria-current="{segment === 'blog' ? 'page' : undefined}" href="blog">blog</a></li> -->
	</ul>
</nav>
