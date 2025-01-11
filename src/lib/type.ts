import type { Selected } from "bits-ui";
import type { Snippet } from "svelte";

export interface Bucket {
  id?: number;
  type: "r2" | "s3";
  bucketName: string;
  accountId: string;
  accessKey: string;
  secretKey: string;
  customDomain: string;
  s3Api?: string;
  [key: string]: string | number | undefined;
}

export interface File {
  type: "text" | "image" | "file";
  id: string;
  source: { filePath: string } | { fileContent: string };
  remoteFilename: string;
  remoteFilenamePrefix: string;
}

export interface FileDetail {
  id: string;
  path: string;
  relativePath: string;
  isDir: boolean;
}

export interface UploadProgress {
  taskId: string;
  filename: string;
  status: UploadStatus;
  timestamp: number;
}

export type UploadStatus =
  | {
      type: "uploading";
      progress: number;
      bytesUploaded: number;
      totalBytes: number;
      speed: number;
    }
  | { type: "success" }
  | { type: "error"; message: string; code: string }
  | { type: "cancelled" };

export interface GlobalState {
  alertMessage: string;
  drag: {
    isDragging: boolean;
    paths: string[];
  };
  modal: ModalState;
  files: Array<File>;
  selectedBucket: Selected<Bucket> | undefined;
  appSetting: AppSettings;
}

export interface AppSettings {
  sidebarCollapsed: boolean;
  useSystemProxy: boolean;
  locale: string;
  defaultBucketId: number | undefined;
}

export interface ModalState {
  isShow: boolean;
  children: Snippet | undefined;
  onClose?: () => void;
}
