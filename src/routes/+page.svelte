<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

// 文件上传相关状态
let endpoint = $state("");
let bucketName = $state("goodasmr");
let accountId = $state("19e4e01d44eb2736350b1c862f0a8a2f");
let accessKey = $state("5ac7a2dae9007729c48bb09f34eccb8a");
let secretKey = $state(
	"673b1f221e57e955b0e93e42013b7045deea8ddb3df6685743a714421f1a1b94",
);
let filePath = $state("");

// 文件上传功能
async function openFile() {
	const file = await open({
		multiple: false,
		directory: false,
	});
	if (file) {
		filePath = file;
	}
}

async function openDirectory() {
	await open({
		multiple: false,
		directory: true,
	});
}

async function uploadFile() {
	try {
		await invoke("r2_upload", {
			bucketName,
			accountId,
			accessKey,
			secretKey,
			filePath,
		});
		console.log("Upload success");
	} catch (error) {
		console.error(error);
	}
}
</script>

<div class="max-w-4xl mx-auto p-6">
  <h1 class="text-2xl font-bold mb-6 text-gray-800">文件上传</h1>

  <div class="bg-white rounded-lg shadow-md p-6">
    <div class="space-y-4">
      <div class="button-group">
        <button onclick="{openFile}" class="px-6 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 active:bg-blue-700 transition-all">选择文件</button>
        <button onclick="{openDirectory}" class="px-6 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 active:bg-blue-700 transition-all">选择文件夹</button>
      </div>
      <button onclick="{uploadFile}" class="w-full px-6 py-2 bg-green-500 text-white rounded-md hover:bg-green-600 active:bg-green-700 transition-all">确认上传</button>
    </div>
  </div>
</div>

<style lang="postcss">
  .form-container {
    @apply max-w-md mx-auto mt-10 p-6 bg-white rounded-lg shadow-md;
  }
  
  .form-title {
    @apply text-2xl font-bold mb-6;
  }
  
  .form-group {
    @apply mb-4;
  }
  
  .form-p {
    @apply block text-sm font-medium mb-1;
  }
  
  .form-input {
    @apply w-full px-3 py-2 border rounded-md;
  }
  
  .form-input:focus {
      @apply outline-none ring-2 ring-blue-500;
  }
  
  .button-group {
      @apply flex gap-4 mt-6;
  }
  
  .action-button {
      @apply px-6 py-2 bg-blue-500 text-white rounded-md transition-all;
  }
  
  .action-button:hover {
      @apply bg-blue-600;
  }
  
  .action-button:active {
      @apply bg-blue-700;
  }
</style>
