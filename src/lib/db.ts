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

class AppDatabase extends Dexie {
  uploadTargets!: Dexie.Table<UploadTarget, number>;

  constructor() {
    super('AppDatabase');
    this.version(1).stores({
      uploadTargets: '++id, name, type'
    });
  }
}

const db = new AppDatabase();

export default db;
