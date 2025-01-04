import Dexie from "dexie";
import type { Bucket } from "./type";

export interface UploadHistory {
  id?: number;
  fileName: string;
  remoteFileName: string;
  target: string;
  timestamp: Date;
}

export interface AppSettings {
  id?: number;
  sidebarCollapsed: boolean;
  useSystemProxy: boolean;
  locale: string;
}

class AppDatabase extends Dexie {
  buckets!: Dexie.Table<Bucket, number>;
  uploadHistory!: Dexie.Table<UploadHistory, number>;
  appSettings!: Dexie.Table<AppSettings, number>;

  constructor() {
    super("AppDatabase");
    this.version(4).stores({
      buckets: "++id, bucketName",
      uploadHistory: "++id, fileName, remoteFileName, target, timestamp",
      appSettings: "++id, sidebarCollapsed, useSystemProxy, locale"
    });
  }
}

const db = new AppDatabase();

export default db;
