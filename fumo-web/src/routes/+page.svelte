<script lang="ts">
  import type { APIFumo } from "$lib";
  import FumoBox from "$lib/FumoBox.svelte";
  import { onMount } from "svelte";

  let fumos: APIFumo[] | null = $state(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      const res = await fetch("https://fumo-api-staging.nosesisaid.com/fumos");
      if (!res.ok) {
        throw new Error("Error while fetching the fumos! "+ await res.text());
      }

      fumos = await res.json();
    } catch (e) {
        alert(e);
    } finally {
        loading = false
    }
  });
</script>

<nav>
  <h2>Fumo-web</h2>
  <a href="https://github.com/Nosesisaid/fumo-api">GitHub</a><a
    href="https://discord.gg/3df68Hg6jF">Discord</a
  >
</nav>

{#if loading}
Loading...
{:else}
{#if fumos}

{#each fumos as fumo}
  <FumoBox {fumo}/>
{/each}
{/if}

{/if}

<style>
    a {
        text-decoration: none;
    }
    nav {
        display: flex;
        gap: 10px;
        align-items: center;
        width: 100%;
        background-color: antiquewhite;
    }
</style>