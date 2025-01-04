import Dexie from "dexie";
import type { Bucket } from "./type";

export interface UploadHistory {
  id?: number;
  fileName: string;
  remoteFileName: string;
  target: string;
  timestamp: Date;
}

class AppDatabase extends Dexie {
  buckets!: Dexie.Table<Bucket, number>;
  uploadHistory!: Dexie.Table<UploadHistory, number>;

  constructor() {
    super("AppDatabase");
    this.version(2).stores({
      buckets: "++id, bucketName",
      uploadHistory: "++id, fileName, remoteFileName, target, timestamp",
    });
  }
}

const db = new AppDatabase();

export default db;
