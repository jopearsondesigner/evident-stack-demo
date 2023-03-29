<script>
  export let data;

  const eventModel = data.eventModel;
  const grid = data.grid;
</script>

<h2>{$eventModel?.EventModel.id}</h2>

<pre>{JSON.stringify($grid)}</pre>
