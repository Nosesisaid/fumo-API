<script lang="ts">
  import type { APIFumo } from "$lib";
  import FumoBox from "$lib/FumoBox.svelte";
  import Pagination from "$lib/Pagination.svelte";
  import { onMount } from "svelte";

  let availablePages = $state(1);
  let page = $state(1);
  let fumos: APIFumo[] | null = $state(null);
  let loading = $state(true);


  onMount(()=>{
    fetch("https://fumo-api-staging.nosesisaid.com/fumos/count").then(r=>{
        r.text().then(t=>{
            availablePages = parseInt(t)
        })
    })
  })

  $effect(() => {
    
    fetch("https://fumo-api-staging.nosesisaid.com/fumos?page="+page).then(r=>{

        if (!r.ok){
            alert("Error loading the fumos"+r.status)
        }
        r.json().then(f => {
            fumos = f as any
            loading = false
        })
    })
    
  });
</script>

<nav>
  <h2>Fumo-web</h2>
  <a href="https://github.com/Nosesisaid/fumo-api">GitHub</a><a
    href="https://discord.gg/3df68Hg6jF">Discord</a
  >
</nav>

<div class="header">
<h1>Fumo?</h1>
<p>Learn more about the fumo-API in our GitHub. Link above.</p>
</div>



{#if loading}
Loading...
{:else}
{#if fumos}

<div class="fumos">
{#each fumos as fumo}
  <FumoBox {fumo}/>
{/each}
</div>
{/if}
{/if}

<Pagination bind:activePage={page} numberOfPages={availablePages} />

<style>

    .fumos {
        margin: 0px 50px;
        display: flex;
        flex-direction: row;
        gap: 20px;
    }
    .header {
        padding: 40px;
    }
    a {
        text-decoration: none;
        color: darkkhaki;
    }
    nav {
        display: flex;
        gap: 10px;
        align-items: center;
        width: 100%;
        background-color: antiquewhite;
        padding: 0px 15px;
    }
</style>