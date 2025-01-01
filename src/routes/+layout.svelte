<script lang="ts">
import "../app.css";
import { onNavigate } from "$app/navigation";
let { children } = $props();

onNavigate((navigation) => {
	if (!document.startViewTransition) return;

	return new Promise((resolve) => {
		document.startViewTransition(async () => {
			resolve();
			await navigation.complete;
		});
	});
});
</script>

<div class="min-h-svh bg-slate-50 dark:bg-slate-900">
	{@render children()}
</div>
