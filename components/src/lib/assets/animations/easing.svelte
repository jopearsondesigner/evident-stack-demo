<script>
	import { flip } from 'svelte/animate';
	import { sineOut, bounceIn, bounceOut, bounceInOut } from 'svelte/easing';

	let [bounceIn_dates, bounceOut_dates, bounceInOut_dates, sineOut_dates] = Array(4).fill(true);

	let dates = ['08-01-2021', '09-01-2021', '10-01-2021', '11-01-2021', '12-01-2021', '13-01-2021'];

	/**
	 * @param {string} number
	 */
	function removeItem(number) {
		dates = dates.filter((n) => n !== number);
	}

	const options = { duration: 600, easing: sineOut };
</script>

<br />

<div class="border">
	<div style="margin-left: 20px;">
		<h1>Flip BounceOut Function</h1>

		<label>
			Dates
			<input type="checkbox" bind:checked={bounceOut_dates} />
		</label>

		{#each dates as d (d)}
			<div
				animate:flip={{ duration: 600, easing: bounceOut }}
				class="container"
				class:bounceOut_dates
			>
				<button on:click={() => removeItem(d)}>{d}</button>
			</div>
		{/each}
	</div>
</div>

<div class="border" style="margin-top: 5px;">
	<div style="margin-left: 20px;">
		<h1>Flip BounceInOut Function</h1>

		<label>
			Dates
			<input type="checkbox" bind:checked={bounceInOut_dates} />
		</label>

		{#each dates as d (d)}
			<div
				animate:flip={{ duration: 600, easing: bounceInOut }}
				class="container"
				class:bounceInOut_dates
			>
				<button on:click={() => removeItem(d)}>{d}</button>
			</div>
		{/each}
	</div>
</div>

<div class="border" style="margin-top: 5px;">
	<div style="margin-left: 20px;">
		<h1>Flip SineOut Function</h1>

		<label>
			Dates
			<input type="checkbox" bind:checked={sineOut_dates} />
		</label>

		{#each dates as d (d)}
			<div animate:flip={options} class="container" class:sineOut_dates>
				<button on:click={() => removeItem(d)}>{d}</button>
			</div>
		{/each}
	</div>
</div>

<div class="border" style="margin-top: 5px;">
	<div style="margin-left: 20px;">
		<h1>Flip BounceIn Function</h1>

		<label>
			Dates
			<input type="checkbox" bind:checked={bounceIn_dates} />
		</label>

		{#each dates as d (d)}
			<div
				animate:flip={{ duration: 600, easing: bounceIn }}
				class="container"
				class:bounceIn_dates
			>
				<button on:click={() => removeItem(d)}>{d}</button>
			</div>
		{/each}
	</div>
</div>

<style>
	.container {
		width: fit-content;
		font-family: Helvetica;
	}

	.bounceIn_dates,
	.bounceOut_dates,
	.bounceInOut_dates,
	.sineOut_dates {
		display: inline-block;
		margin-left: 10px;
	}

	.border {
		border-style: solid;
		box-shadow: #cccccc;
		width: 50%;
		border-color: coral;
		border-width: thin;
	}
</style>
