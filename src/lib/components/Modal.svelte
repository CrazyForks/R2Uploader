<script lang="ts">
  import { modalState } from "$lib/store.svelte";
  import { onMount } from "svelte";

  let modal: HTMLDialogElement;
  let isLoading = $state(true);

  onMount(() => {
    requestAnimationFrame(() => {
      isLoading = false;
    });
  });

  function onclose() {
    modalState.isShow = false;
    modalState.children = undefined;
  }

  $effect(() => {
    if (modalState.isShow) {
      modal.showModal();
    } else {
      modal.close();
    }
  });
</script>

<dialog
  id="modal"
  class="modal modal-bottom sm:modal-middle"
  bind:this={modal}
  {onclose}
>
  <div class="modal-box dark:bg-slate-800">
    {#if isLoading}
      <div class="flex h-full items-center justify-center p-2">loading...</div>
    {:else}
      {@render modalState.children?.()}
    {/if}
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>
