import { type File } from "./type";
export let filesState: { files: Array<File> } = $state({ files: [] });

export function clearFiles() {
  filesState.files = [];
}
