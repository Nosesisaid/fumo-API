<script lang="ts">
    export let numberOfPages = 1
    export let activePage = 1


function handleClick(n:number){
    activePage = n
}
</script>

<div class="pagination">
{#each {length:numberOfPages-1} as _,i}
    <button class:active={ i+1 === activePage} on:click={_=>handleClick(i+1)}>{i+1}</button>
{/each}
</div>

<style>
    .pagination {
        width: 100%;
        align-content: center;
        display: inline-block;
        margin: 30px 40px;
    }
    .pagination button {
        cursor: pointer;
        color: black;
        float: left;
        background-color: transparent;
        padding: 10px 10px;
        margin-right: 10px;
    }
    .pagination button.active {
        background-color: beige;
    }
</style>