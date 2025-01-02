<script lang="ts">
import "../app.css";
import Sidebar from "$lib/components/Sidebar.svelte";
import Alert from "$lib/components/Alert.svelte";
let { children } = $props();
</script>

<Alert />

<div class="flex h-screen bg-slate-50 dark:bg-slate-900">
  <Sidebar />
  <main class="flex-1 overflow-y-auto">
    {@render children()}
  </main>
</div>
