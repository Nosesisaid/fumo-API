<script lang="ts">
  import type { APIFumo } from "$lib";

    export let fumo: APIFumo = null as unknown as APIFumo
</script>

<div class="container">
        <img
      src={fumo.img}
      alt={`Image of a Touhou series plush Caption: ${fumo.caption}`}
    />
    <div class="infocolumn">
    <t>#{fumo.id}</t>
    <t>Featuring: <i>{(fumo.involved.length > 0 ? fumo.involved.join(" ") : "No one")}</i></t>
    <i></i>
    </div>
</div>

<style>
    .infocolumn {
        padding: 0px 10px;
        display: flex;
        flex-direction: column;
    }
    t {
        color: white;
    }
    .container {
        background-color: rgb(103, 136, 38);
        
        width: 250px;
        padding: 10px;
        flex-direction: column;
        display: flex;
    }
    .container img {
        margin: 10px 5px ;
    }
</style>