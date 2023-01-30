<script>
  /* @type {number} */
  export let row = 0;

  /* @type {number} */
  export let column = 0;

  /* @type {boolean} */
  export let cursor = false;
</script>

<div
  class="empty-cell z-10 self-stretch relative"
  style="grid-row: {row} / {row}; grid-column: {column} / {column};"/>
