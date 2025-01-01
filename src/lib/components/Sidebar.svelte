<script lang="ts">
import {
	History,
	Plus,
	PanelRightOpen,
	PanelRightClose,
	Settings,
} from "lucide-svelte";
import { page } from "$app/state";
let collapsed = $state(false);
</script>

<nav
	class="min-h-dvh border-r border-slate-200 bg-white p-2 transition-all dark:border-slate-700 dark:bg-slate-800 {collapsed
		? 'w-16'
		: 'w-40'}"
>
	<ul class="flex flex-col items-center justify-center space-y-2">
		<li class="flex w-full items-center {collapsed ? 'justify-center' : 'justify-end'}">
			<button onclick={() => (collapsed = !collapsed)} class="sidebar-link cursor-pointer">
				{#if !collapsed}
					<PanelRightOpen class="size-5" />
				{:else}
					<PanelRightClose class="size-5" />
				{/if}
			</button>
		</li>
		<li>
			<a
				href="/"
				class="sidebar-link"
				aria-current={page.route.id === '/' ? 'page' : null}
			>
				<Plus class="size-5" />
				{#if !collapsed}
					<span class="text-nowrap">Upload</span>
				{/if}
			</a>
		</li>
		<li>
			<a
				href="/setting"
				class="sidebar-link"
				aria-current={page.route.id === '/setting' ? 'page' : null}
			>
				<Settings class="size-5" />
				{#if !collapsed}
					<span class="text-nowrap">Setting</span>
				{/if}
			</a>
		</li>
	</ul>
</nav>

<style lang="postcss">
	.sidebar-link {
		@apply flex items-center gap-3 rounded-xl px-4 py-3 text-gray-700 transition-colors hover:bg-slate-100 dark:text-gray-200 dark:hover:bg-slate-700;
	}

	.sidebar-link[aria-current] {
		@apply bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400;
	}
</style>
