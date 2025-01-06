import type { Selected } from "bits-ui";
import { type Bucket, type File } from "./type";

export let filesState: { files: Array<File> } = $state({ files: [] });

export let bucketsState: { selected: Selected<Bucket> | undefined } = $state({
  selected: undefined,
});
