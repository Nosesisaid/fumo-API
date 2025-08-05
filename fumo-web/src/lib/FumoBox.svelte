<script lang="ts">
  import type { APIFumo } from "$lib";

    export let fumo: APIFumo = null as unknown as APIFumo
</script>

<div class="container">
        <img
      src={fumo.img}
      alt={`Image of a Touhou series plush Caption: ${fumo.caption}`}
    />
    <i>{fumo.id}</i>
</div>