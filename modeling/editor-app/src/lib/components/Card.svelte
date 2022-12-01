<script>
 /* @type {"interface" | "command" | "event" | "readModel" } */
 export let type_ = "command";
 /* @type {string} */
 export let label = "Card";
</script>

<div class="card card-compact bg-{type_} shadow-xl">
    <div class="card-body">
        {label}
    </div>
</div>
