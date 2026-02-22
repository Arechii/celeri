<script lang="ts">
  import { createGetFood } from "$lib";

  const foodsQuery = createGetFood();
</script>

{#if foodsQuery.isLoading}
  Loading...
{:else if foodsQuery.isError}
  Error loading food
{:else}
  {foodsQuery.data?.data.name}
{/if}
