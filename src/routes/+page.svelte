<script>
import { Dashboard } from "@uppy/svelte";
import Uppy from "@uppy/core";

// Don't forget the CSS: core and UI components + plugins you are using
import "@uppy/core/dist/style.css";
import "@uppy/dashboard/dist/style.css";
import "@uppy/file-input/dist/style.css";
import FileInput from "@uppy/file-input";

const uppy = new Uppy().use(FileInput);
</script>

<main><Dashboard uppy={uppy} /></main>