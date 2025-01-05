export interface Bucket {
  id?: number;
  type: "r2" | "s3";
  bucketName: string;
  accountId: string;
  accessKey: string;
  secretKey: string;
  customDomain: string;
  [key: string]: string | number | undefined;
}

export interface File {
  type: "text" | "image" | "file";
  id: string;
  source: { filePath: string } | { fileContent: string };
  remoteFilename: string;
  remoteFilenamePrefix: string;
  selected: boolean;
}
