import Dexie from 'dexie';

export interface UploadTarget {
  id?: number;
  name: string;
  description: string;
  type: 'r2';
  bucketName: string;
  accountId: string;
  accessKey: string;
  secretKey: string;
}

export interface UploadHistory {
  id?: number;
  fileName: string;
  remoteFileName: string;
  target: string;
  timestamp: Date;
}

class AppDatabase extends Dexie {
  uploadTargets!: Dexie.Table<UploadTarget, number>;
  uploadHistory!: Dexie.Table<UploadHistory, number>;

  constructor() {
    super('AppDatabase');
    this.version(2).stores({
      uploadTargets: '++id, name, type',
      uploadHistory: '++id, fileName, remoteFileName, target, timestamp'
    });
  }
}

const db = new AppDatabase();

export default db;
